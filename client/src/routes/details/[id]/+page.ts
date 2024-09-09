import axios from "axios"
import type { PageLoad } from "./$types"
import type { Todo } from "../../../types/todo"
import { PUBLIC_API_URL } from "$env/static/public"

export const load: PageLoad = async ({ params }) => {
        const response = await axios.get(
                `${PUBLIC_API_URL}/api/todos/${params.id}`
        )

        return response.data as Todo
} 
