# Password Check

**Install Projects**

```bash
git clone git@github.com:gjhonic/PasswordCheck.git
```

**Install Docker**

```bash
sudo pacman -S docker
sudo pacman -S docker-compose
```

**launch**

```bash
# Start docker (not work in windows wsl)
sudo systemctl start docker

#if you use windows wsl then need command
sudo service docker start

cd PasswordCheck

make upd
```

**Test**

*Open page in browser: http://127.0.0.1/*

**Other command**
```bash

make

```