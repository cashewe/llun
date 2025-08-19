import json


with open('data/schema.json', 'r') as f:
    schema = json.load(f)
    
formatted_schema = json.dumps(schema, indent=2)

with open('data/system_prompt.txt', 'r', encoding='utf-8') as file:
    system_prompt = file.read().format(formatted_schema)

with open('data/user_prompt.txt', 'r', encoding='utf-8') as file:
    user_prompt = file.read()
