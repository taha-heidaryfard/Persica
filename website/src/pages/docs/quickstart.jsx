import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';
import { Play, Sparkles, Terminal } from 'lucide-react';

export default function QuickstartPage() {
  const meta = {
    title: 'First Program & Quickstart',
    category: 'Getting Started',
    description: 'Write, execute, and inspect your first Persica application in under two minutes.',
  };

  const toc = [
    { title: 'Zero-Ceremony Philosophy', id: 'philosophy', level: 2 },
    { title: 'Hello World Script', id: 'hello-world', level: 2 },
    { title: 'Interactive Input & Output', id: 'io', level: 2 },
    { title: 'A Complete Example: Fibonacci', id: 'fibonacci', level: 2 },
    { title: 'Next Steps', id: 'next-steps', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="philosophy" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Zero-Ceremony Philosophy
      </h2>
      <p>
        Unlike many compiled languages like Java or C++ that require boilerplate classes, package headers, and complex project directory scaffolding just to print a message, Persica follows the <strong>zero-ceremony</strong> philosophy:
      </p>
      <p className="text-emerald-300 font-medium">
        Any valid Persica statement at the top-level of a file can be executed instantly.
      </p>

      <h2 id="hello-world" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Hello World Script
      </h2>
      <p>
        Create a new file called <code className="text-emerald-300">hello.persi</code> in any directory:
      </p>

      <CodeBlock
        language="persica"
        filename="hello.persi"
        code={`// Your first Persica script
print("Hello, world! 🌿");
print("Welcome to Persica version 0.13.0");`}
      />

      <p>Execute it directly from your terminal:</p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`persica hello.persi`}
        output={`Hello, world! 🌿
Welcome to Persica version 0.13.0`}
      />

      <h2 id="io" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Interactive Input & Output
      </h2>
      <p>
        Persica provides Python-style <code className="text-emerald-400">input(prompt)</code> and multi-argument <code className="text-emerald-400">print()</code> with custom separators:
      </p>

      <CodeBlock
        language="persica"
        filename="greeting.persi"
        code={`// Prompt the user for their name
string name = input("Enter your name: ");

// Read an integer age and convert with to_int
int age = to_int(input("Enter your age: "));

print("Hello", name, "!");
print("In 5 years, you will be", age + 5, "years old.");`}
        output={`Enter your name: Alex
Enter your age: 25
Hello Alex !
In 5 years, you will be 30 years old.`}
      />

      <h2 id="fibonacci" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        A Complete Example: Fibonacci Numbers
      </h2>
      <p>
        Here is a complete program demonstrating loops, arrays, typed variables, and formatting:
      </p>

      <CodeBlock
        language="persica"
        filename="fibonacci.persi"
        code={`// Function to generate the first n Fibonacci numbers
int[] generate_fibonacci(int count) {
    int[] fib = [0, 1];
    
    for (int i = 2; i < count; i++) {
        int next_val = fib[i - 1] + fib[i - 2];
        append(fib, next_val);
    }
    
    return fib;
}

int count = 10;
print("Generating first", count, "Fibonacci numbers:");

int[] result = generate_fibonacci(count);

for idx in range(0, len(result)) {
    print("Fib(" + to_string(idx) + ") =", result[idx]);
}`}
        output={`Generating first 10 Fibonacci numbers:
Fib(0) = 0
Fib(1) = 1
Fib(2) = 1
Fib(3) = 2
Fib(4) = 3
Fib(5) = 5
Fib(6) = 8
Fib(7) = 13
Fib(8) = 21
Fib(9) = 34`}
      />

      <h2 id="next-steps" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Next Steps
      </h2>
      <p>
        Now that you know how to run scripts, continue exploring the rest of Persica’s feature set:
      </p>
      <ul className="text-xs text-slate-300 list-disc pl-5 space-y-1.5 my-3">
        <li>Learn how to scaffold full multi-file applications with <a href="/Persica/docs/project-system" className="text-emerald-400 font-semibold underline">persica.toml & .persi_env</a>.</li>
        <li>Review Persica’s static type system in <a href="/Persica/docs/syntax-basics" className="text-emerald-400 font-semibold underline">Variables & Data Types</a>.</li>
        <li>Master object-oriented programming in <a href="/Persica/docs/oop" className="text-emerald-400 font-semibold underline">Classes & Structs</a>.</li>
      </ul>
    </DocsLayout>
  );
}
