import { Component } from 'solid-js';
import { useFormHandler } from 'solid-form-handler';
import { zodSchema } from 'solid-form-handler/zod';

import { z } from 'zod';
import TextInput from '../InputComponents/TextInput';
import Button from '../InputComponents/Button';
import { onSubmitHandler } from '../utils/forms';
import { newUser, NewUserPayload } from '../stores/authStore';
import { useNavigate } from '@solidjs/router';
import { fieldErrors } from '../utils/fieldValidation';

export const userSchema = z.object({
  username: z
    .string({ required_error: fieldErrors.required })
    .min(4, fieldErrors.minLength(4))
    .max(12, fieldErrors.maxLength(12)),
  password: z
    .string({ required_error: 'El campo es requerido' })
    .min(8, fieldErrors.minLength(8)),
});

type CreateUserProps = {
  isAdmin?: true;
};

const CreateUser: Component<CreateUserProps> = ({ isAdmin = false }) => {
  const formHandler = useFormHandler(zodSchema(userSchema));
  const navigate = useNavigate();

  const { formData } = formHandler;

  const onSubmit = async (event: Event) => {
    const payload = {
      ...formData(),
      role: isAdmin ? 'admin' : 'user',
    } satisfies NewUserPayload;

    onSubmitHandler(event, formHandler, newUser, payload).then(() => {
      navigate('/');
    });
  };

  return (
    <form class="flex flex-col pt-6 px-8 lg:px-[35%]" onSubmit={onSubmit}>
      <div class="pb-4">
        <TextInput
          label="Nombre de usuario"
          name="username"
          formHandler={formHandler}
        />
        <TextInput
          label="Contraseña"
          name="password"
          formHandler={formHandler}
        />
      </div>
      <Button
        text="Submit"
        variant="blue"
        disabled={formHandler.isFormInvalid}
      />
    </form>
  );
};

export default CreateUser;
