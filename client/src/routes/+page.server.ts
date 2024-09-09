import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import axios from 'axios';
import { PUBLIC_API_URL } from '$env/static/public';

export const actions: Actions = {
        default: async (event) => {
                const formData = await event.request.formData();

                const title = formData.get("title")?.toString();
                const body = formData.get("body")?.toString();

                // TODO: real validation or sumn
                if (!title?.trim() || !body?.trim()) return fail(400, { error: "u can't do that bruv" })

                await axios.post(`${PUBLIC_API_URL}/api/todos`, { title, body })

                return { status: "success", message: "yessiiirrr" }
        }
};
