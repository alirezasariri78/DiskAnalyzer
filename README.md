# DiskAnalyzer

## Description

Check Folder Size And Create Diagram Based On Their Size

![image description](pics/debian_tree.png)

## Note:

- Project May Not Work On Some Of System Folders Due To Lack Of Permissions (Access Denied)
- This Project Works On Windows And Linux Only

## How To Use :

```

get information about size of folders in each drive

Usage: DiskAnalyzer.exe [OPTIONS]

Options:
  -d, --drive <drives>     which drive to scan.
                           split with space.
                            scan all drives if not set
      --depth <drive>      how many level of inner directories should it scan
  -p, --path <path>        analyze give path
      --diagram <diagram>  Set Diagram Types : tree , bar
  -h, --help               Print help
  -V, --version            Print version

```

## ToDO List :

- [x] Add Depth For Folder Crawler
- [ ] Add More Diagrams
- [ ] Add Mac Support (Currently It Supports Windows And Linux)
- [ ] Get Largest Files In Disk
- [ ] Handle Files In Terminal (Add Move , etc)
- [ ] List File Types (Audio , Video , etc)
