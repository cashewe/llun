import json


with open('schema.json', 'r') as f:
    schema = json.load(f)
    
formatted_schema = json.dumps(schema, indent=2)

prompt = f"""you are a linting tool that will be used for either python code projects or technical documentation. your purpose is not to focus on 'code quality', spelling, grammar etc... but rather on ensuring the architecture of the solution is inline with the technical strategy of the organisation. This technical strategy is described using the following rules, which you are to interpret as plain text.
```
{}
```
you are also provided the following additional free text context by the organisation:
```
{}
```
scan the code for violations of the provided strategic rules, keeping the additional context in mind. respond using the following schema, keeping to multiple separate violations of each code where a user repeatedly commits the sin rather than lumping them all into one: 
```
{formatted_schema}
```
the code for you to scan is as follows:
```
{}
```
"""