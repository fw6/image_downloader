# Move to the root of the repository
cd "$(dirname "$0")"
cd ../../

if [ "$1" != "-r" ]; then
  # Dev
  cargo build -p imagekit -p imagekit-wasm --target wasm32-unknown-unknown --features wasm-bindgen
  wasm-bindgen --target bundler --out-dir languages/wasm ./target/wasm32-unknown-unknown/debug/imagekit_wasm.wasm
  wasm-bindgen --target nodejs --out-dir languages/wasm/node ./target/wasm32-unknown-unknown/debug/imagekit_wasm.wasm
else
  # Release
  cargo build -p imagekit -p imagekit-wasm --target wasm32-unknown-unknown --features wasm-bindgen --release
  wasm-bindgen --target bundler --out-dir languages/wasm ./target/wasm32-unknown-unknown/release/imagekit_wasm.wasm
  wasm-bindgen --target nodejs --out-dir languages/wasm/node ./target/wasm32-unknown-unknown/release/imagekit_wasm.wasm
fi

# Optimize size
wasm-opt -Os ./languages/wasm/imagekit_wasm_bg.wasm -o ./languages/wasm/imagekit_wasm_bg.wasm
wasm-opt -Os ./languages/wasm/node/imagekit_wasm_bg.wasm -o ./languages/wasm/node/imagekit_wasm_bg.wasm

# Transpile to JS
wasm2js ./languages/wasm/imagekit_wasm_bg.wasm -o ./languages/wasm/imagekit_wasm_bg.wasm.js
npx terser ./languages/wasm/imagekit_wasm_bg.wasm.js -o ./languages/wasm/imagekit_wasm_bg.wasm.js
