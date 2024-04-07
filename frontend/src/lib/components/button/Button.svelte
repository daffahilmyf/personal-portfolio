<script>
	/**
	 * Changes the button background / outline color.
	 * @type {(import('$components/types').color)}
	 */
	export let color = 'bg-gradient-red';

	/** @type {string} */
	export let name = '';

	/** @type {boolean} */
	export let outlined = false;

	/** @type {boolean} */
	export let roundedFull = true;

	/** @type {import('$components/types').textColor} */
	export let textColor = 'text-white';

	/** @type {import('./types').textSize} */
	export let textSize = 'text-[1.125rem]';

	/** @type {boolean} */
	export let textOnly = false;

	const buttonOutlined = `p-1 ${color}`;
	const buttonPrimary = `px-5 py-[0.625rem] ${color}`;
	const buttonTextOnly = 'bg-white px-5 py-[0.625rem]';
	const buttonContentOutlined = 'bg-white flex flex-col justify-center px-4 py-[0.375rem]';
</script>

<button
	class="font-poppins font-bold h-fit inline-block w-fit {textSize} {outlined
		? buttonOutlined
		: textOnly
			? buttonTextOnly
			: buttonPrimary}"
	class:rounded-full={roundedFull}
	{name}
	on:click
>
	<div class={outlined ? buttonContentOutlined : ''} class:rounded-full={roundedFull}>
		<div class={outlined || textOnly ? `bg-clip-text text-transparent ${color}` : textColor}>
			<slot>Button</slot>
		</div>
	</div>
</button>
