# antimodules
antimodules is a simple rust utility to remove all node_modules folders from a given directory or the execution directory by default. 

## usage
To use antimodules simply run cargo install and the github repository. 
```
$ cargo install --git https://github.com/noreplydev/antimodules
```
Then run the command antimodules in the directory you want to remove all node_modules folders from. 
```
$ antimodules
```

#### Search faster (ignore folders)

If you want to ignore some folders on the path you can use the -i (--ignore) flag. 
```
$ antimodules -i src,tests mydevfolder
```

The -i flag takes a comma separated list of folder names to ignore. The last argument of the command is the path to the directory you want to remove all node_modules folders from. If no path is given the current working directory is used.

#### Help

To get help with the command use the -h (--help) flag. 
```
$ antimodules -h
```

#### Version

To get the version of the command use the -v (--version) flag. 
```
$ antimodules -v
```

## Contributing
If you want to contribute to antimodules feel free to open a pull request. Feel free to contact me via email or linkedin (available on https://cristian-sanchez.me).

@noreplydev 
