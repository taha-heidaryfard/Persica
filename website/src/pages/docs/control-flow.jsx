import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';

export default function ControlFlowPage() {
  const meta = {
    title: 'Control Flow & Loops',
    category: 'Core Language',
    description: 'Guide to conditional branching, while loops, Pythonic for-in iteration, range generation, and loop control in Persica.',
  };

  const toc = [
    { title: 'Conditionals: `if` and `else`', id: 'if-else', level: 2 },
    { title: 'Pythonic `for ... in` Loops', id: 'for-in', level: 2 },
    { title: 'The `range()` Generator', id: 'range', level: 2 },
    { title: '`while` Loops', id: 'while', level: 2 },
    { title: 'C-Style Indexed `for` Loops', id: 'c-for', level: 2 },
    { title: '`break` and `continue`', id: 'break-continue', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="if-else" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Conditionals: `if` and `else`
      </h2>
      <p>
        In Persica, parentheses around conditions are optional. You can write clean Python-like conditions or traditional C-style:
      </p>

      <CodeBlock
        language="persica"
        filename="conditions.persi"
        code={`int score = 88;

if score >= 90 {
    print("Grade: A");
} else if score >= 80 {
    print("Grade: B");
} else if score >= 70 {
    print("Grade: C");
} else {
    print("Grade: F");
}`}
        output={`Grade: B`}
      />

      <h2 id="for-in" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Pythonic `for ... in` Loops
      </h2>
      <p>
        Persica supports direct iteration over arrays and strings using the <code className="text-emerald-400">for item in collection</code> syntax:
      </p>

      <CodeBlock
        language="persica"
        filename="for_in.persi"
        code={`// Iterating over an array
string[] fruits = ["Apple", "Orange", "Pomegranate"];

for fruit in fruits {
    print("Fruit:", fruit);
}

// Iterating over a Unicode UTF-8 string character by character
string greeting = "سلام";
for char in greeting {
    print("Char:", char);
}`}
        output={`Fruit: Apple
Fruit: Orange
Fruit: Pomegranate
Char: س
Char: ل
Char: ا
Char: م`}
      />

      <h2 id="range" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        The `range()` Generator
      </h2>
      <p>
        The built-in <code className="text-emerald-400">range()</code> function produces a sequence of integers and accepts 1, 2, or 3 arguments:
      </p>

      <div className="overflow-x-auto my-4 text-xs">
        <table className="w-full text-left border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">Invocation</th>
              <th className="p-3">Generated Values</th>
              <th className="p-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400">range(5)</td>
              <td className="p-3 font-mono">[0, 1, 2, 3, 4]</td>
              <td className="p-3">From 0 up to stop (exclusive).</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">range(1, 6)</td>
              <td className="p-3 font-mono">[1, 2, 3, 4, 5]</td>
              <td className="p-3">From start up to stop (exclusive).</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">range(0, 10, 2)</td>
              <td className="p-3 font-mono">[0, 2, 4, 6, 8]</td>
              <td className="p-3">With custom step increment.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">range(5, 0, -1)</td>
              <td className="p-3 font-mono">[5, 4, 3, 2, 1]</td>
              <td className="p-3">With negative countdown step.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <CodeBlock
        language="persica"
        filename="range_demo.persi"
        code={`// Counting down with step -1
print("Countdown starting:");
for i in range(3, 0, -1) {
    print(i, end="... ");
}
print("Liftoff! 🚀");`}
        output={`Countdown starting:
3... 2... 1... Liftoff! 🚀`}
      />

      <h2 id="while" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        `while` Loops
      </h2>
      <p>
        Execute a block of statements repeatedly as long as the boolean condition remains true:
      </p>

      <CodeBlock
        language="persica"
        filename="while_demo.persi"
        code={`int n = 1;
while n < 32 {
    print("Power of two:", n);
    n *= 2;
}`}
        output={`Power of two: 1
Power of two: 2
Power of two: 4
Power of two: 8
Power of two: 16`}
      />

      <h2 id="c-for" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        C-Style Indexed `for` Loops
      </h2>
      <p>
        For classic counter-based iteration with explicit initializers, conditions, and update statements:
      </p>

      <CodeBlock
        language="persica"
        filename="c_style.persi"
        code={`for (int i = 0; i < 3; i++) {
    print("Index:", i);
}`}
        output={`Index: 0
Index: 1
Index: 2`}
      />

      <h2 id="break-continue" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        `break` and `continue`
      </h2>
      <p>
        Use <code className="text-emerald-400">break</code> to exit a loop early, or <code className="text-emerald-400">continue</code> to jump immediately to the next iteration:
      </p>

      <CodeBlock
        language="persica"
        filename="jump.persi"
        code={`for i in range(1, 10) {
    if i % 2 == 0 {
        continue; // skip even numbers
    }
    if i > 7 {
        break; // stop when exceeding 7
    }
    print("Odd:", i);
}`}
        output={`Odd: 1
Odd: 3
Odd: 5
Odd: 7`}
      />
    </DocsLayout>
  );
}
