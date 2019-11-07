# Maintainer: wint3rmute <mateusz.baczek1998@gmail.com>
pkgname=tnb
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="command | tnb - pipe your commands to your Telegram Notifications Bot"
license=('WTFPL')

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" tnb
}
