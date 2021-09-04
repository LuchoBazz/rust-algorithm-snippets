import os
import json

HOME = os.path.expanduser("~") 
PATH = HOME + '/.config/Code/User/snippets/rust.json'

def add_snippets_to_vscode(root_path, filename, ext, exclude):
    if filename in exclude:
        return
    
    path = root_path + filename + '.' + ext
    prefix_path = root_path.split('/')
    prefix_path.pop()
    prefix_path = prefix_path.pop()
    
    json_str = ''
    with open(PATH, 'r') as json_data:
        json_str += json_data.read()

    snippets = json.loads(json_str)
    code = []
    with open(path, 'r') as reader:
        code = reader.read().split('\n')
    
    for i in range(len(code)):
        line = code[i]
        line = line.replace("$", "\\$")
        code[i] = line

    if 'template_' in filename:
        for i in range(len(code)):
            line = code[i]
            line = line.replace(
                '`!v strftime("%B %d, %Y")`',
                '${CURRENT_MONTH_NAME} ${CURRENT_DATE}, ${CURRENT_YEAR}'
            )
            code[i] = line
    
    data = {
        'prefix': prefix_path + '_' + filename,
        'body': code
    }
    
    snippets[prefix_path + '_' + filename] = data

    snippets = json.dumps(snippets)
    snippets = json.loads(snippets)

    print('Snippet', filename, 'was generated')
    with open(PATH, 'w') as outfile:
        json.dump(snippets, outfile,  indent=4)

def main():
    basepath = './src/'
    for directory in os.listdir(basepath):
        if os.path.isdir(os.path.join(basepath, directory)):
            dir_path = basepath + directory + '/'
            
            for filename in os.listdir(dir_path):
                fragments = filename.split('.')
                ext = fragments.pop()
                filename = ''
                if len(fragments) > 0:
                    filename = fragments.pop()
                else:
                    continue

                if ext == 'rs' and filename != '':
                    add_snippets_to_vscode(
                        dir_path,
                        filename,
                        ext,
                        exclude=['mod', 'lib']
                    )

if __name__ == '__main__':
    main()