# What is RAW
Raw is a project of a package manager from scratch for Onyx.

# Features : 
- Building packages from Pkgfiles
- Installing, removing updating these packages
- Files and packages conflict detection
- Listing libs installed by a package 
- Searching ownership of a file with query
- Installing packages from remote repo 
- Setting up an index for the remote repo to work
- Mode binary and mode source allowing to also use building in a transparent way
- Listing installed packages
- Bootstraping packages into a folder or within a given path
- Detecting maximum core available for building.
- Some locks to not mess up your installation.
- Only linked to the kernel, glibc and xz at runtime
- Pre and post installation support


# Pkgfile exemple :
``` bash
description=" The Nano package contains a small, simple text editor which aims to replace Pico, the default editor in the Pine package. "
name=nano
version=9.0
release=1
packager=alexis
source=(https://www.nano-editor.org/dist/v9/nano-${version}.tar.xz)
depends="kernel-headers libpng"
build() {
cd $name-$version
./configure --prefix=/usr     \
            --sysconfdir=/etc \
            --enable-utf8     \
            --docdir=/usr/share/doc/nano-${version} &&
make
make DESTDIR=$PKG install
install -v -m644 doc/{nano.html,sample.nanorc} $PKG/usr/share/doc/nano-${version}
}
```

# Example of raw.conf 
``` bash 
alexis [ ~/Onyx ]$ cat /etc/raw.conf 
mode binary

root=/home/alexis/Onyx

url=https://remoterepo
alexis [ ~/Onyx ]$ 

alexis [ ~/Onyx ]$ cat /etc/raw.conf 
mode source

root=/home/alexis/Onyx
alexis [ ~/Onyx ]$ 
```

# Example of post/pre-installation file name 
pkgname.post-install
pkgname.pre-install


# How to build it ? 
Download the latest release, uncompress the tarball and enter the directory.
Run :
``` bash 
cargo build --release
sudo cp target/release/raw /usr/bin/
sudo touch /etc/raw.conf
``` 
You're now all set ! 

# Where to find already working Pkgfiles for my lfs system ? 
Take a look at :
<https://github.com/Delta-Azura/onyx>

# Educational purpose
My target is to be able to learn rust while building this projet, it will maybe never be able to be usable for Onyx.
I've just started rust, i'm still very young but i want to perform in this field.