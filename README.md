Rust for C++, Java, Python and Nodejs.

# Preposition

Install nodejs
```bash
# install nvm
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.2/install.sh | bash
# load nvm config
source ~/.$(basename $SHELL)rc
# install Node.js 8
nvm install 8
# use Node.js 8
nvm use 8
# install ffi
cd rust_ffi
npm install node-gyp 
npm install ffi ref ref-array ref-struct
```

# Run
```bash
./run.sh
```