addEventListener('fetch', event => {
	event.respondWith(handleRequest(event.request));
});

async function handleRequest(req) {
	const { handle_request } = wasm_bindgen;
	await wasm_bindgen(wasm);
	const output = handle_request(request.url);

	return new Response(output, {
		status: 200,
		headers: {'content-type': 'image/svg+xml'},
	});
}
