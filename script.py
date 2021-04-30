import os

# function to copy files to new location with foldername as filename
def copyfile(file_name, new_name, scripts_folder):
    with open(file_name, 'r') as rf:
        contents = rf.read()
        with open(os.path.join(os.path.join(cwd,scripts_folder), new_name), 'w') as wf:
            wf.write(contents)


cwd = os.getcwd()
scripts_folder = 'rust_scripts'

# Creating the scripts_folder if does not exist.
try: os.mkdir(scripts_folder)
except: print(f"'{scripts_folder}' folder already created!")

# getting the folder_names that are later to be used as rust code file names
list_dir = [fldr for fldr in os.listdir(cwd) if fldr != scripts_folder]

# loop to repeat the copy function
for folder in list_dir:
    try:
        rust_file = os.path.join(os.path.join(cwd, folder),
         'src/main.rs')
        print(f'The rust code "{rust_file}" is copied to rust_scripts folder as "{folder}.rs" file')
        copyfile(rust_file,
                (folder+'.rs'), scripts_folder)
    except:
        continue