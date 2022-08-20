// Register a service worker
console.log('[App] Register a Service worker')
if ('serviceWorker' in navigator) {
	navigator.serviceWorker.register('service-worker.js');
	console.log('[App] Succesfully registered a service worker')
} else {
	console.log('[App] The browser does not support Service workers')
}