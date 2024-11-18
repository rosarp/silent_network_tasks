import websocketHandler from './websocket.js';

document.addEventListener('alpine:init', () => {
    Alpine.data('websocketHandler', websocketHandler);
});
