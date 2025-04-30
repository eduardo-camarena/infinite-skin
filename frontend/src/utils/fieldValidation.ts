export const fieldErrors = {
	required: 'El campo es requerido',
	minLength: (length: number) =>
		`El campo debe tener al menos ${length} caracteres`,
	maxLength: (length: number) =>
		`El campo debe no tener mas de ${length} caracteres`,
};
