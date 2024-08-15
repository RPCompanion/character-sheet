
import json
import pyjson5

def main():
    
    with open("standard.json5", "r") as fp:

        content = pyjson5.load(fp)

        with open("standard.json", "w") as fp:
            json.dump(content, fp, indent=4)

if __name__ == '__main__':
    main()