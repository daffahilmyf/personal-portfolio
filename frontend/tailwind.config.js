/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme');

export default {
	content: ['./src/**/*.{html,js,svelte}'],
	theme: {
		colors: {
			black: {
				50: '#fbfbfb',
				100: '#f2f2f2',
				200: '#ededed',
				300: '#c0c0c0',
				400: '#949494',
				500: '#848080',
				600: '#5d5d5d',
				700: '#2e2e2e',
				800: '#292929',
				900: '#121212',
				950: '#000000'
			},
			red: {
				100: '#fce8e7'
			},
			white: '#ffffff'
		},
		extend: {
			backgroundImage: {
				'gradient-red': 'linear-gradient(81.79deg, #e51510 24.63%, #ff7370 91.16%)',
				'gradient-black': 'linear-gradient(107.27deg, #646464 32.42%, #000000 114.61%)'
			},
			fontFamily: {
				josefin: ['"Josefin Sans"', ...defaultTheme.fontFamily.sans],
				montserrat: ['Montserrat', ...defaultTheme.fontFamily.sans],
				poppins: ['Poppins', ...defaultTheme.fontFamily.sans]
			}
		}
	},
	plugins: []
};
