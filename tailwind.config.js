/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			colors: {
				primary: "#1a1a1a",
				secondary: "#222",
				tertiary: "#AED2FF",
				quaternary: "#E4F1FF"
			}
		}
	},
	plugins: []
};
