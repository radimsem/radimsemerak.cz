<script>
	// ui
	import '@material/web/button/filled-button.js';
	import '@material/web/button/filled-tonal-button.js';

	// styles
	import '../styles/index.css';
</script>

<div class="app overflow-x-hidden">
	<div class="back-layer fixed w-full h-screen z-0"></div>

	<slot />
</div>

<style>
	:root {
		--md-sys-typescale-label-large-font: system-ui;
		--md-filled-button-container-color: #4f46e5;
		--md-sys-color-on-secondary-container: rgb(226 232 240);
		--md-filled-tonal-button-container-color: rgb(32,39,55);
	}

	* {
		scroll-behavior: smooth;
	}

	.back-layer {
		background: linear-gradient(
			175deg, 
			rgb(18,24,27) 50%, 
			rgb(32,39,55) 100%
		);
	}
</style>
