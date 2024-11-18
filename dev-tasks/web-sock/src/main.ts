import './style.css'
import init, { wsPing } from '../src-wasm/pkg'

// Declare the global function type
declare global {
  interface Window {
    callWsEndpoint: () => Promise<void>;
  }
}

window.addEventListener('load', async () => {
  await init()
    .then(() => {
      console.log('WASM module initialized successfully');
    })
    .catch(error => {
      console.error('Failed to initialize WASM module:', error);
    });
});

// Make the function available globally to be called from the HTML
window.callWsEndpoint = async function() {
  // Get the input values
  const endpointInput = document.getElementById('endpointInput') as HTMLInputElement;
  const endpoint = endpointInput.value;
  const messageInput = document.getElementById('messageInput') as HTMLInputElement;
  const message = messageInput.value;

  // Call the WASM function to connect with the WebSocket server
  // and handle the response
  await wsPing(endpoint, message)
  .then(response => {
    console.log('Received response:', response);
    var wsResponseLabel = document.getElementById('wsResponseLabel') as HTMLInputElement;
    wsResponseLabel.textContent = response;
  })
  .catch(error => {
    console.error('WebSocket error:', error);
  });
}
