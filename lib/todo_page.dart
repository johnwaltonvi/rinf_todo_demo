import 'package:flutter/material.dart';
import 'src/bindings/bindings.dart';

class TodoPage extends StatefulWidget {
  const TodoPage({super.key});

  @override
  State<TodoPage> createState() => _TodoPageState();
}

class _TodoPageState extends State<TodoPage> {
  final TextEditingController _textController = TextEditingController();

  @override
  void initState() {
    super.initState();
    // Request initial todos when the page loads
    const TodoCommandGetAll().sendSignalToRust();
  }

  @override
  void dispose() {
    _textController.dispose();
    super.dispose();
  }

  void _addTodo() {
    if (_textController.text.isNotEmpty) {
      TodoCommandAdd(text: _textController.text).sendSignalToRust();
      _textController.clear();
    }
  }

  void _toggleTodo(int id) {
    TodoCommandToggle(id: id).sendSignalToRust();
  }

  void _deleteTodo(int id) {
    TodoCommandDelete(id: id).sendSignalToRust();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Todo App'),
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _textController,
                    decoration: const InputDecoration(
                      hintText: 'Add a new todo...',
                      border: OutlineInputBorder(),
                    ),
                    onSubmitted: (_) => _addTodo(),
                  ),
                ),
                const SizedBox(width: 16),
                ElevatedButton(
                  onPressed: _addTodo,
                  child: const Text('Add'),
                ),
              ],
            ),
          ),
          // Use StreamBuilder to listen for TodoList signals from Rust
          StreamBuilder(
            stream: TodoList.rustSignalStream,
            builder: (context, snapshot) {
              final signalPack = snapshot.data;
              if (signalPack == null) {
                return const Expanded(
                  child: Center(
                    child: Text('Loading todos...'),
                  ),
                );
              }

              final todoList = signalPack.message;
              final todos = todoList.items;
              final pendingCount = todoList.pendingCount;

              return Expanded(
                child: Column(
                  children: [
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16.0),
                      child: Row(
                        children: [
                          Text(
                            'Pending tasks: $pendingCount',
                            style: const TextStyle(
                              fontWeight: FontWeight.bold,
                              fontSize: 16,
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Divider(),
                    Expanded(
                      child: ListView.builder(
                        itemCount: todos.length,
                        itemBuilder: (context, index) {
                          final todo = todos[index];
                          return ListTile(
                            leading: Checkbox(
                              value: todo.completed,
                              onChanged: (_) => _toggleTodo(todo.id),
                            ),
                            title: Text(
                              todo.text,
                              style: TextStyle(
                                decoration: todo.completed
                                    ? TextDecoration.lineThrough
                                    : TextDecoration.none,
                              ),
                            ),
                            trailing: IconButton(
                              icon: const Icon(Icons.delete),
                              onPressed: () => _deleteTodo(todo.id),
                            ),
                          );
                        },
                      ),
                    ),
                  ],
                ),
              );
            },
          ),
        ],
      ),
    );
  }
}
