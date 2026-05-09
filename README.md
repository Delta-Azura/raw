# What is RAW
Raw is a project of a package manager from scratch for Onyx.
It's designed to be quick, light and memory safe.

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
- Support of the prepare and package functions alongside with the build=something type of Pkgfiles


# Pkgfile example :
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

# Example of build=something Pkgfile : 
``` bash
alexis [ ~/htop ]$ cat Pkgfile 
description=" The htop package provides a TUI [1] system monitor and has became well-known for its ease-of-use and comprehensive features. "
name=htop
version=3.5.1
release=1
packager=alexis
depends="nano"
source=("https://github.com/htop-dev/htop/releases/download/${version}/htop-${version}.tar.xz")
```
If this space is left empty, raw will therefore search in the /etc/raw.d/ directory for a file named build-default, it should look like this : 
''' bash
alexis [ ~/htop ]$ cat /etc/raw.d/build-default 
cd $name-$version
./configure --prefix=/usr &&
make
make DESTDIR=$PKG install 
```

You can also use this template of Pkgfile : 
``` bash 
alexis [ ~/htop ]$ cat Pkgfile 
description=" The htop package provides a TUI [1] system monitor and has became well-known for its ease-of-use and comprehensive features. "
name=htop
version=3.5.1
release=1
packager=alexis
depends="nano"
source=("https://github.com/htop-dev/htop/releases/download/${version}/htop-${version}.tar.xz")

build=make
``` 
In this case, raw will try to look at make, defined by build here, in the /etc/raw.d/make file.


# WARNING 
Raw automatically detects if a function prepare or package is present on the Pkgfile, however, do not leave a package or prepare or build function with a '#' at the start of the line, raw will not detect it, run the function commented and the build will fail.

# How to build it ? 
Download the latest release, uncompress the tarball and enter the directory.
Run :
``` bash 
cargo build --release
sudo cp target/release/raw /usr/bin/
sudo touch /etc/raw.conf
``` 
You're now all set ! 

# Basic usage :
``` bash
raw package # Build a package being in a repertory containing a valid Pkgfile
sudo raw install htop.3.5.1#1.raw.tar.gz # Install the generated package
raw info htop # To get the basic informations
raw query /usr/bin/htop # To know who this file belongs to
raw libs htop # To list every libraries owned by htop
raw libs systemd all # To list every libraries owned by systemd including the /security libs
raw list # List every packages installed
sudo raw update htop.3.5.1#1.raw.tar.gz # To update htop
raw index # In source mode it allows you to generate an index that will be useful for the next command
raw build htop # Will work no matter the path you are at and will propose to install/update it 
sudo raw upgrade # Allows you to upgrade your systemd if you are using a remote binary repo
sudo raw get htop # Allows you to install packages from a remote binary repo and handles dependencies
```

# Where to find already working Pkgfiles for my lfs system ? 
Take a look at :
<https://github.com/Delta-Azura/onyx>

# Educational purpose
My target is to be able to learn rust while building this projet.
From all the tests i ran, not a single one is actually failing no matter what you are trying to do with it.
I built this is one week as a learning project.
It's a fully functional package manager, 0 compilation error and it handles conflict detection, dependecy resolution and so on, please refer to the beggining of the README.