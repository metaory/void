#!/bin/bash

# Migrate from old binary protobuf format to new JSON format
# Input: binary protobuf file
# Output: JSON array [nodes, max_id, arrows] where:
#   nodes: array of [id, meta, text, children, collapsed, stricken, hide_stricken, parent_id, free_text]
#   max_id: highest node ID
#   arrows: array of arrow connections (empty in migration)

FORCE=0

while [[ $# -gt 0 ]]; do
  case $1 in
    -f|--force)
      FORCE=1
      shift
      ;;
    *)
      if [[ -z "$INPUT" ]]; then
        INPUT="$1"
      elif [[ -z "$OUTPUT" ]]; then
        OUTPUT="$1"
      else
        echo "Error: Too many arguments"
        echo "Usage: $0 [-f|--force] input_file [output_file]"
        exit 1
      fi
      shift
      ;;
  esac
done

[[ -z "$INPUT" ]] && { echo "Usage: $0 [-f|--force] input_file [output_file]"; exit 1; }
[[ -f "$INPUT" ]] || { echo "Error: Input file not found"; exit 1; }
[[ -n "$OUTPUT" && -f "$OUTPUT" && $FORCE -eq 0 ]] && { echo "Error: Output file already exists (use -f to overwrite)"; exit 1; }

# Check if input is already JSON
if head -c1 "$INPUT" | grep -q '^[[:space:]]*\['; then
  echo "Error: Input file appears to be JSON already. Use -f to force migration." >&2
  exit 1
fi

function migrate {
  # Use protoc to decode binary format, then transform to JSON
  # The output format matches the app's JsonScreen type:
  # type JsonScreen = (Vec<JsonNode>, u64, Vec<(u64, u64)>)
  # where JsonNode = (id, meta, text, children, collapsed, stricken, hide_stricken, parent_id, free_text)
  protoc --decode_raw < "$1" | awk -v meta='{"ctime":0,"mtime":0,"finish_time":null,"tags":[],"due":null}' '
    BEGIN { 
      max_id = 0
      first = 1
      print "["
    }
    
    /^1 {/ { 
      node_id = 0
      node_text = ""
      children = ""
      parent_id = 0
      next 
    }

    /^}/ { 
      if (!first) printf ","
      printf "[%d,%s,\"%s\",%s,false,false,false,%d,null]", 
        node_id, meta, node_text, children ? children "]" : "[]", parent_id
      if (node_id > max_id) max_id = node_id
      first = 0
      next
    }
    
    /^  1:/ { node_id = $2; next }
    /^  3:/ { node_text = substr($0, index($0, ":") + 2); gsub(/"/, "", node_text); next }
    /^  4:/ { 
      if (children == "") children = "["
      else children = children ","
      children = children $2
      next 
    }
    /^  11:/ { parent_id = $2; next }
    
    END {
      print "]"
      print max_id
      print "[]"
    }
  ' | jq -s '[.[0], .[1], .[2]]'
}

if [[ -n "$OUTPUT" ]]; then
  migrate "$INPUT" > "$OUTPUT" || { echo "Error: Migration failed"; exit 1; }
  echo -e "\nSuccessfully migrated $INPUT to $OUTPUT"
  echo -e "\nTo use the migrated database, run:"
  echo "  void $OUTPUT"
else
  migrate "$INPUT"
fi 