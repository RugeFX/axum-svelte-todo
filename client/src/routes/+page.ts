import type { PageLoad } from "./$types"

interface Todo {
        id: number,
        title: string,
        body: string
}

export const load: PageLoad = async ({ fetch }) => {
        const response = await fetch(
                "http://localhost:3000/api/todos"
        )

        const todos: Todo[] = await response.json()
        return { todos }
} 
