import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';
import { Cpu, Layers, Sparkles } from 'lucide-react';

export default function StdlibPage() {
  const meta = {
    title: 'Standard Library & Auto-Prelude',
    category: 'Object-Oriented & Robustness',
    description: 'Explore Persica’s self-hosted standard library and the always-available Prelude data structures, written 100% in Persica.',
  };

  const toc = [
    { title: 'The Self-Hosted Standard Library', id: 'self-hosted', level: 2 },
    { title: 'The Auto-Prelude', id: 'prelude', level: 2 },
    { title: '`Stack` & `Queue`', id: 'stack-queue', level: 2 },
    { title: '`Set` & `StringBuilder`', id: 'set-sb', level: 2 },
    { title: '`Result` & `Option` Containers', id: 'result-option', level: 2 },
    { title: 'Standard Math Module (`std.math`)', id: 'math-module', level: 2 },
    { title: 'Standard Collections (`std.collections`)', id: 'collections-module', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="self-hosted" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        The Self-Hosted Standard Library
      </h2>
      <p>
        Persica practices <strong>100% dogfooding</strong>: every high-level data structure and algorithm in the standard library is implemented directly in Persica source code (under <code className="text-emerald-400">src/stdlib/</code>), compiled into the final binary with zero runtime overhead.
      </p>

      <h2 id="prelude" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        The Auto-Prelude
      </h2>
      <p>
        The <strong>Prelude</strong> is a collection of essential data structures and utility functions that are <strong>automatically in scope</strong> in every Persica program without requiring an <code className="text-emerald-300 font-semibold">import</code> statement:
      </p>

      <h2 id="stack-queue" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        `Stack` & `Queue`
      </h2>
      <CodeBlock
        language="persica"
        filename="stack_queue.persi"
        code={`// 1. Stack (LIFO: Last In, First Out)
Stack stack = Stack();
stack.push("Layer 1");
stack.push("Layer 2");
stack.push("Layer 3");

print("Popped item:", stack.pop());
print("Top item (peek):", stack.peek());
print("Stack size:", stack.len());

// 2. Queue (FIFO: First In, First Out)
Queue queue = Queue();
queue.enqueue("Customer 1");
queue.enqueue("Customer 2");

print("Served first:", queue.dequeue());
print("Remaining in queue:", queue.len());`}
        output={`Popped item: Layer 3
Top item (peek): Layer 2
Stack size: 2
Served first: Customer 1
Remaining in queue: 1`}
      />

      <h2 id="set-sb" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        `Set` & `StringBuilder`
      </h2>
      <CodeBlock
        language="persica"
        filename="set_sb.persi"
        code={`// 1. Set (Unique items collection)
Set unique_ids = Set();
unique_ids.add("USR-101");
unique_ids.add("USR-102");
unique_ids.add("USR-101"); // Duplicate ignored!

print("Set size:", unique_ids.len());
print("Contains USR-102?", unique_ids.contains("USR-102"));

// 2. StringBuilder (Efficient string buffering)
StringBuilder sb = StringBuilder();
sb.append_str("SELECT * FROM users ");
sb.append_str("WHERE status = 'active' ");
sb.append_line("ORDER BY id DESC;");

print("Constructed SQL:");
print(sb.to_string());`}
        output={`Set size: 2
Contains USR-102? true
Constructed SQL:
SELECT * FROM users WHERE status = 'active' ORDER BY id DESC;`}
      />

      <h2 id="result-option" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        `Result` & `Option` Containers
      </h2>
      <p>
        Modeled after Rust’s idiomatic functional types:
      </p>

      <CodeBlock
        language="persica"
        filename="result_option.persi"
        code={`// Safe division using Result
Result safe_divide(int a, int b) {
    if b == 0 {
        return err("Cannot divide by zero");
    }
    return ok(a / b);
}

Result res = safe_divide(100, 5);
if res.is_ok {
    print("Computation succeeded:", res.value);
} else {
    print("Computation failed:", res.error);
}`}
        output={`Computation succeeded: 20`}
      />

      <h2 id="math-module" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Standard Math Module (`std.math`)
      </h2>
      <CodeBlock
        language="persica"
        filename="math_demo.persi"
        code={`from std.math import sqrt, pow, gcd, is_prime;

print("Square root of 81.0:", sqrt(81.0));
print("2 raised to power 8:", pow(2.0, 8.0));
print("Greatest common divisor of 54 and 24:", gcd(54, 24));
print("Is 97 prime?", is_prime(97));`}
        output={`Square root of 81.0: 9
2 raised to power 8: 256
Greatest common divisor of 54 and 24: 6
Is 97 prime? true`}
      />

      <h2 id="collections-module" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Standard Collections (`std.collections`)
      </h2>
      <CodeBlock
        language="persica"
        filename="algorithms_demo.persi"
        code={`from std.collections import zip, chunks, binary_search;

int[] numbers = [10, 20, 30, 40, 50, 60];

// Break array into chunks of 2
print("Chunks of 2:", chunks(numbers, 2));

// Fast binary search on sorted array
print("Index of 40 via binary search:", binary_search(numbers, 40));`}
        output={`Chunks of 2: [[10, 20], [30, 40], [50, 60]]
Index of 40 via binary search: 3`}
      />
    </DocsLayout>
  );
}
