import tomllib
import tomli_w
import requests

file_path = './languages.toml'
url = 'https://raw.githubusercontent.com/helix-editor/helix/master/languages.toml'

response = requests.get(url)
response.raise_for_status()

with open(file_path, 'wb') as file:
    file.write(response.content)

with open(file_path, 'rb') as file:
    data = tomllib.load(file)

output = {
    'language-server': {
        'discord-presence': {
            'command': 'discord-presence-lsp'
        }
    },
    'language': []
}

if 'language' in data:
    for lang in data['language']:
        if 'language-servers' in lang:
            lang['language-servers'].append("discord-presence")
        else:
            lang['language-servers'] = ["discord-presence"]
            
        output['language'].append({
            'name': lang['name'],
            'language-servers': lang['language-servers']
        })

with open(file_path, 'wb') as file:
    file.write(b"# For Discord Presence\n")
    tomli_w.dump(output, file)