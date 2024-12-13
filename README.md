# QuickR
An easy-to-use QR code generator web app.

## Available Scripts

In the project directory, you can run:

### `npm run start`
Runs the React development server at [localhost:3000](http://localhost:3000/).
### `npm run build:react`
Builds the React side of the project.
### `npm run build:wasm`:
Builds the WASM side of the project.
### `npm run build`
Builds the entire project. (`build:wasm` + `build:react`)
### `npm run deploy`
Installs additional tools for deployment puroposes.

## Technologies Used

### Rust
All QR code generation is handled in [Rust](https://www.rust-lang.org/) using [fast_qr](https://github.com/erwanvivien/fast_qr).\
Visit [the official page](https://www.rust-lang.org/learn) to check out various ways to learn Rust!

### WebAssembly
In order to boost performance, this project uses [WebAssembly](https://webassembly.org).\
The [wasm-pack](https://github.com/rustwasm/wasm-pack) and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) were used in order to integrate React, Rust and WASM.

### React
This project relies on [React](https://react.dev/) for the visuals. To learn React, check out the [React documentation](https://reactjs.org/).\
Additionally, I used [Create React App](https://github.com/facebook/create-react-app) to bootstrap the project.

## Credits
While I built this application, I didn't do it all by myself. This project wouldn't be possible without all of the open-source technology it's built on.\
Special thanks to [tkat0](https://github.com/tkat0) for the helpful article on [setting up Rust and WebAssembly with React](https://www.tkat0.dev/posts/how-to-create-a-react-app-with-rust-and-wasm/). Additionally, I found [this page](https://www.barcodefaq.com/2d/qr-code/) invaluable (especially [this section](https://www.barcodefaq.com/2d/qr-code/#Common_Uses)) in learning qr code syntax.
