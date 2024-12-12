# QuickR

An easy-to-use QR code generator web app.

## Available Scripts

In the project directory, you can run:

### `npm start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in your browser.

The page will reload when you make changes.\
You may also see any lint errors in the console.

### `npm test`

Launches the test runner in the interactive watch mode.\
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

### `npm run build`

Builds the app for production to the `build` folder.\
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.\
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

### `npm run eject`

**Note: this is a one-way operation. Once you `eject`, you can't go back!**

If you aren't satisfied with the build tool and configuration choices, you can `eject` at any time. This command will remove the single build dependency from your project.

Instead, it will copy all the configuration files and the transitive dependencies (webpack, Babel, ESLint, etc) right into your project so you have full control over them. All of the commands except `eject` will still work, but they will point to the copied scripts so you can tweak them. At this point you're on your own.

You don't have to ever use `eject`. The curated feature set is suitable for small and middle deployments, and you shouldn't feel obligated to use this feature. However we understand that this tool wouldn't be useful if you couldn't customize it when you are ready for it.

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
Special thanks to [tkat0](https://github.com/tkat0) for the helpful on [setting up Rust and WebAssembly with React](https://www.tkat0.dev/posts/how-to-create-a-react-app-with-rust-and-wasm/). Additionally, I found [this page](https://www.barcodefaq.com/2d/qr-code/) invaluable (especially [this section](https://www.barcodefaq.com/2d/qr-code/#Common_Uses)) in learning qr code syntax.
