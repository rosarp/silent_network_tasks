export default () => ({
    showMessenger: false,
    connected: false,
    connectButtonText: 'Connect',
    isTimedOut: false,
    socket: null,
    channelId: '',
    message: '',
    notifications: [],
    timeoutId: null,
    init() {
        this.$watch('socket', (socket) => {
            if (socket) {
                socket.onopen = () => {
                    this.connectButtonText = 'Waiting for second party...';
                    // Start 10 second timeout if no response from second party
                    this.timeoutId = setTimeout(() => {
                        if (this.notifications.length === 0) {
                            this.isTimedOut = true;
                            this.socket.close();
                        }
                    }, 10000);
                }
                socket.onmessage = (event) => {
                    // Clear timeout on first message
                    this.clearTimeout();
                    this.connectButtonText = 'Disconnect';
                    this.showMessenger = true;
                    this.notifications.push(event.data);
                }
                socket.onclose = () => {
                    // Clear timeout on close
                    this.clearTimeout();
                    this.resetData();
                }
            }
        });
        this.$watch('isTimedOut', (isTimedOut) => {
            if (isTimedOut) {
                this.resetData();
            }
        });
    },
    connect() {
        if (!this.connected) {
            this.socket = new WebSocket(`ws://localhost:3000/wait-for-second-party/${this.channelId}`);
            this.connected = true;
            this.showMessenger = true;
        } else {
            this.socket.close();
        }
    },
    sendMessage() {
        if (this.socket && this.message.trim()) {
            this.socket.send(this.message);
            this.message = '';
        }
    },
    clearTimeout() {
        if (this.timeoutId) {
            clearTimeout(this.timeoutId);
            this.timeoutId = null;
        }
    },
    resetData() {
        this.showMessenger = false;
        this.connected = false;
        this.connectButtonText = 'Connect';
        this.channelId = '';
        this.message = '';
        this.notifications = [];
        this.socket = null;
    }
});
