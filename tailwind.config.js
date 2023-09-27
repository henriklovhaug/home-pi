/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			colors: {
				primary: "#1f1f1f",
				secondary: "#9400FF",
				tertiary: "#AED2FF",
				quaternary: "#E4F1FF"
			}
		}
	},
	plugins: []
};
