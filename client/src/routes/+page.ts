import type { Todo } from "../types/todo"
import type { PageLoad } from "./$types"
import { PUBLIC_API_URL } from "$env/static/public"

export const load: PageLoad = async ({ fetch }) => {
        const response = await fetch(
                `${PUBLIC_API_URL}/api/todos`
        )

        const todos: Todo[] = await response.json()
        return { todos }
} 
