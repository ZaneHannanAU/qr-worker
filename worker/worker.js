addEventListener('fetch', event => {
	event.respondWith(
		handleRequest(event.request)
		.catch(e => new Response(JSON.stringify(e.stack), {status: 500}))
	);
});
const stat = async n => new Response(
	await fetch(`https://http.cat/${n}.jpg`),
	{status: n}
);
async function handleRequest(req) {
	if (req.url.length > 140) return stat(414);

	const { handle_request } = wasm_bindgen;
	await wasm_bindgen(wasm);
	const output = handle_request(req.url);

	return new Response(output, {
		status: 200,
		headers: {'content-type': 'image/svg+xml'},
	});
}
