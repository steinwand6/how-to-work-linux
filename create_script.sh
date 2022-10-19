#!/usr/bin/bash 
echo '#!/usr/bin/env rust-script' > $1
echo '//! ```cargo ' >> $1
echo '//! [dependencies] ' >> $1
echo '//! nix = "0.25.0" ' >> $1
echo '//! ``` ' >> $1
echo '' >> $1
echo 'fn main() {' >> $1
echo '' >> $1
echo '}' >> $1

chmod +x $1
