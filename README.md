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
- Parallel source download -> binaries parallel download coming soon
- Support of makedepends


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

# Example of raw.conf 
``` bash 
alexis [ ~/Onyx ]$ cat /etc/raw.conf 
mode=binary

root=/home/alexis/Onyx

url=https://remoterepo
alexis [ ~/Onyx ]$ 

alexis [ ~/Onyx ]$ cat /etc/raw.conf 
mode=source

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
raw search htop # Allows you to search for packages both in binary and source mode
```

# Where to find already working Pkgfiles for my lfs system ? 
Take a look at :
<https://github.com/Delta-Azura/onyx>

# If you are reading this from git.great-os.org/alexis/raw
Report any issue and open a pull request only to : 
<https://github.com/Delta-Azura/raw>

# Educational purpose
My target is to be able to learn rust while building this projet.
From all the tests i ran, not a single one is actually failing no matter what you are trying to do with it.
I built this is one week as a learning project.
It's a fully functional package manager, 0 compilation error and it handles conflict detection, dependecy resolution and so on, please refer to the beggining of the README.


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
- Téléchargement parallèle des sources -> téléchargement parallèle des binaires à venir
- Support des makedepends

# Exemple de Pkgfile :
```bash
description=" The Nano package contains a small, simple text editor which aims to replace Pico, the default editor in the Pine package. "
name=nano
version=9.0
release=1
packager=alexis
source=(https://www.nano-editor.org/dist/v9/nano-${version}.tar.xz)
makedepends="htop" #Ceci est un exemple
rundepends="kernel-headers libpng" #Ceci est un exemple pour montrer la syntaxe
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

# Exemple de raw.conf
```bash
alexis [ ~/Onyx ]$ cat /etc/raw.conf
mode=binary

root=/home/alexis/Onyx

url=https://remoterepo
alexis [ ~/Onyx ]$

alexis [ ~/Onyx ]$ cat /etc/raw.conf
mode=source

root=/home/alexis/Onyx
alexis [ ~/Onyx ]$
```

# Exemple de nom de fichier post/pre-installation
```
pkgname.post-install
pkgname.pre-install
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

# ATTENTION
Raw détecte automatiquement si une fonction prepare ou package est présente dans le Pkgfile. Ne laissez cependant pas une fonction package, prepare ou build avec un '#' en début de ligne — raw ne la détectera pas, exécutera la fonction commentée et la compilation échouera.

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
```

# Où trouver des Pkgfiles fonctionnels pour mon système LFS ?
Jetez un œil à :
<https://github.com/Delta-Azura/onyx>

# Si vous lisez ceci depuis git.great-os.org/alexis/raw
Signalez les problèmes et ouvrez des pull requests uniquement sur :
<https://github.com/Delta-Azura/raw>

# Objectif pédagogique
Mon objectif est d'apprendre Rust en construisant ce projet.
D'après tous les tests que j'ai effectués, aucun n'échoue quelle que soit l'opération tentée.
Je l'ai construit en une semaine comme projet d'apprentissage.
C'est un gestionnaire de paquets pleinement fonctionnel, 0 erreur de compilation, avec détection des conflits, résolution des dépendances, etc. Référez-vous au début du README pour plus de détails.
