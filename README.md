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
- /etc preserved
- Display symlinks in footprints
- Diff of footprints between builds 
- Parallel source download -> binaries parallel download coming soon
- Support of makedepends
- Support of integrity of packages 
- Changelog function 
- Support of aur like repositories
- Support of dependencies while removing, type sudo raw remove pkgname -f to force.
- Automatic detection of runtime dependencies scanning the binary.
- Automatic post-installation hooks
- RAW_KEEP_SOURCES=false to remove the sources after building the package
- Parallel download for sources (yet to arrive for binary mode)


# Pkgfile example :
``` bash
description=" The Nano package contains a small, simple text editor which aims to replace Pico, the default editor in the Pine package. "
name=nano
version=9.0
release=1
packager=alexis
source=(https://www.nano-editor.org/dist/v9/nano-${version}.tar.xz)
makedepends="htop" #This is an example
rundepends="kernel-headers libpng" #This is an example to show the syntax
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

# WARNING 
Raw automatically detects if a function prepare or package is present on the Pkgfile, however, do not leave a package or prepare or build function with a '#' at the start of the line, raw will not detect it, run the function commented and the build will fail.


# Example of raw.conf 
``` bash 
alexis [ ~/Onyx ]$ cat /etc/raw.conf 
mode=binary

source=/home/alexis/Onyx

url=https://remoterepo

community=link

local=true # If you want to enable a local repository

alexis [ ~/Onyx ]$ 

alexis [ ~/Onyx ]$ cat /etc/raw.conf 
mode=source

root=/home/alexis/Onyx

community=link
alexis [ ~/Onyx ]$ 
```

# Example of post/pre-installation file name 
pkgname.post-install
pkgname.pre-install

Example of post/pre-removal file name
pkgname.post-remove
pkgname.pre-remove

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
``` bash
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
sudo raw remove package # Remove the package
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
raw search htop # Allows you to search for packages both in binary and source mode
sudo raw bootstrap package path # To install the binaries into a folder
sudo raw community packagename # To install a package from an aur repo like
sudo raw rmcache # To remove the cached packages
sudo raw orphans # to list all of the orphans packages
```

# Where to find already working Pkgfiles for my lfs system ? 
Take a look at :
<https://github.com/Delta-Azura/onyx>

# A little fetcher to see outdated pkgfiles : 
<https://github.com/Delta-Azura/raw-fetch>

# If you are reading this from git.great-os.org/alexis/raw
Report any issue and open a pull request only to : 
<https://github.com/Delta-Azura/raw>

# Warning : 
There are a lot of functionnality that hasn't been tested so far. 
The listed functions above should work perfectly has they are correctly tested, but some like local package support in binary mode might not work properly. 
The changelog function in binary mode isn't tested yet.

# Qu'est-ce que RAW
Raw est un projet de gestionnaire de paquets from scratch pour Onyx.
Il est conçu pour être rapide, léger et memory-safe.

# Fonctionnalités :
- Compilation de paquets depuis des Pkgfiles
- Installation, suppression et mise à jour de ces paquets
- Détection des conflits entre fichiers et paquets
- Listage des libs installées par un paquet
- Recherche du propriétaire d'un fichier via une requête
- Installation de paquets depuis un dépôt distant
- Mise en place d'un index pour le dépôt distant
- Mode binaire et mode source permettant d'utiliser la compilation de manière transparente
- Listage des paquets installés
- Bootstrap de paquets dans un dossier ou dans un chemin donné
- Détection du nombre maximum de cœurs disponibles pour la compilation
- Verrous pour ne pas corrompre votre installation
- Uniquement lié au kernel, glibc et xz à l'exécution
- Support des scripts pre et post installation
- Support des fonctions prepare et package ainsi que du type de Pkgfile build=quelquechose
- /etc préservé
- Affichage des symlinks dans les footprints
- Diff des footprints entre deux builds
- Téléchargement parallèle des sources -> téléchargement parallèle des binaires à venir
- Support des makedepends
- Support de l'intégrité des paquets
- Fonction changelog
- Support des dépôts façon AUR
- Support des dépendances à la suppression, taper sudo raw remove pkgname -f pour forcer
- Détection automatique des dépendances runtime par scan du binaire
- Hooks post-installation automatiques
- RAW_KEEP_SOURCES=false pour supprimer les sources après avoir construit le paquet.
- Parallel download for sources (pas encore disponible pour le mode binaire)


# Exemple de Pkgfile :
```bash
description=" The Nano package contains a small, simple text editor which aims to replace Pico, the default editor in the Pine package. "
name=nano
version=9.0
release=1
packager=alexis
source=(https://www.nano-editor.org/dist/v9/nano-${version}.tar.xz)
makedepends="htop" #This is an example
rundepends="kernel-headers libpng" #This is an example to show the syntax
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

# ATTENTION
Raw détecte automatiquement si une fonction prepare ou package est présente dans le Pkgfile. Ne laissez cependant pas une fonction package, prepare ou build avec un '#' en début de ligne — raw ne la détectera pas, exécutera la fonction commentée et la compilation échouera.

# Exemple de raw.conf
```bash
alexis [ ~/Onyx ]$ cat /etc/raw.conf
mode=binary

source=/home/alexis/Onyx

url=https://remoterepo

community=link

local=true # Si tu veux activer un dépôt local

alexis [ ~/Onyx ]$

alexis [ ~/Onyx ]$ cat /etc/raw.conf
mode=source

root=/home/alexis/Onyx

community=link
alexis [ ~/Onyx ]$
```

# Exemple de nom de fichier post/pre-installation
```
pkgname.post-install
pkgname.pre-install
```

Exemple de nom de fichier post/pre-removal
```
pkgname.post-remove
pkgname.pre-remove
```

# Exemple de Pkgfile avec build=quelquechose :
```bash
alexis [ ~/htop ]$ cat Pkgfile
description=" The htop package provides a TUI [1] system monitor and has became well-known for its ease-of-use and comprehensive features. "
name=htop
version=3.5.1
release=1
packager=alexis
depends="nano"
source=("https://github.com/htop-dev/htop/releases/download/${version}/htop-${version}.tar.xz")
```
Si ce champ est laissé vide, raw cherchera dans le répertoire /etc/raw.d/ un fichier nommé build-default, qui doit ressembler à ceci :
```bash
alexis [ ~/htop ]$ cat /etc/raw.d/build-default
cd $name-$version
./configure --prefix=/usr &&
make
make DESTDIR=$PKG install
```

Vous pouvez également utiliser ce template de Pkgfile :
```bash
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
Dans ce cas, raw cherchera make, défini par build ici, dans le fichier /etc/raw.d/make.

# Comment le compiler ?
Téléchargez la dernière release, décompressez l'archive et entrez dans le répertoire.
Exécutez :
```bash
cargo build --release
sudo cp target/release/raw /usr/bin/
sudo touch /etc/raw.conf
```
Vous êtes prêt !

# Utilisation basique :
```bash
raw package # Compile un paquet en étant dans un répertoire contenant un Pkgfile valide
sudo raw install htop.3.5.1#1.raw.tar.gz # Installe le paquet généré
sudo raw remove package # Supprime le paquet
raw info htop # Obtenir les informations de base
raw query /usr/bin/htop # Savoir à qui appartient ce fichier
raw libs htop # Lister toutes les bibliothèques appartenant à htop
raw libs systemd all # Lister toutes les bibliothèques de systemd y compris les libs /security
raw list # Lister tous les paquets installés
sudo raw update htop.3.5.1#1.raw.tar.gz # Mettre à jour htop
raw index # En mode source, génère un index utile pour la commande suivante
raw build htop # Fonctionne peu importe le chemin et proposera d'installer/mettre à jour
sudo raw upgrade # Permet de mettre à jour vos paquets si vous utilisez un dépôt binaire distant
sudo raw get htop # Installe des paquets depuis un dépôt binaire distant et gère les dépendances
raw search htop # Recherche des paquets en mode binaire et source
sudo raw bootstrap package path # Pour installer les binaires dans un dossier
sudo raw community packagename # Pour installer un paquet depuis un dépôt façon AUR
sudo raw rmcache # Pour supprimer les paquets en cache
sudo raw orphans # Pour lister tous les paquets orphelins
```

# Où trouver des Pkgfiles fonctionnels pour mon système LFS ?
Jetez un œil à :
<https://github.com/Delta-Azura/onyx>

# Un petit récupérateur pour repérer les pkgfiles obsolètes :
<https:/github.com/Delta-Azura/raw-fetch>

# Si vous lisez ceci depuis git.great-os.org/alexis/raw
Signalez les problèmes et ouvrez des pull requests uniquement sur :
<https://github.com/Delta-Azura/raw>

# Attention :
Beaucoup de fonctionnalités n'ont pas encore été testées à fond.
Les fonctions listées ci-dessus devraient fonctionner parfaitement dès qu'elles sont correctement testées, mais certaines comme le support des paquets locaux en mode binaire pourraient ne pas fonctionner correctement.
La fonction changelog en mode binaire n'est pas encore testée.