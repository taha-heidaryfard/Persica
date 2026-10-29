import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';

export default function CollectionsPage() {
  const meta = {
    title: 'Arrays, Maps & Dictionaries',
    category: 'Core Language',
    description: 'Complete guide to collection data structures in Persica: typed arrays, dynamic multidimensional lists, maps, and dictionaries.',
  };

  const toc = [
    { title: 'Typed Arrays', id: 'arrays', level: 2 },
    { title: 'Multi-dimensional Arrays', id: 'multidimensional', level: 2 },
    { title: 'Built-in Array Helpers', id: 'helpers', level: 2 },
    { title: 'Dictionaries (`dict`)', id: 'dict', level: 2 },
    { title: 'Typed Maps (`map<K, V>`)', id: 'maps', level: 2 },
    { title: 'Keys, Values & Inspection', id: 'inspection', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="arrays" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Typed Arrays
      </h2>
      <p>
        Arrays in Persica are indexed from zero, grow dynamically, and can be typed to enforce element consistency:
      </p>

      <CodeBlock
        language="persica"
        filename="arrays.persi"
        code={`// Array of integers
int[] numbers = [10, 20, 30, 40];

// Accessing and mutating by index
print("First element:", numbers[0]);
numbers[1] = 99; // mutate index 1

// Negative indexing access from the back
print("Last element via -1:", numbers[-1]);
print("Updated array:", numbers);`}
        output={`First element: 10
Last element via -1: 40
Updated array: [10, 99, 30, 40]`}
      />

      <h2 id="multidimensional" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Multi-dimensional Arrays
      </h2>
      <p>
        Nested arrays are created by repeating bracket pairs (e.g. <code className="text-emerald-400">int[][]</code>):
      </p>

      <CodeBlock
        language="persica"
        filename="matrix.persi"
        code={`int[][] matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

print("Center element [1][1]:", matrix[1][1]);
matrix[0][2] = 99;
print("Row 0 updated:", matrix[0]);`}
        output={`Center element [1][1]: 5
Row 0 updated: [1, 2, 99]`}
      />

      <h2 id="helpers" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Built-in Array Helpers
      </h2>
      <div className="overflow-x-auto my-4 text-xs">
        <table className="w-full text-left border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">Function</th>
              <th className="p-3">Return</th>
              <th className="p-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400">len(arr)</td>
              <td className="p-3 font-mono">int</td>
              <td className="p-3">Returns count of elements.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">append(arr, item)</td>
              <td className="p-3 font-mono">void</td>
              <td className="p-3">Adds element to the end of array in-place.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">sort(arr)</td>
              <td className="p-3 font-mono">array</td>
              <td className="p-3">Returns a new sorted copy (ascending).</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">reverse(arr)</td>
              <td className="p-3 font-mono">array</td>
              <td className="p-3">Returns a new reversed copy.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">slice(arr, start, end)</td>
              <td className="p-3 font-mono">array</td>
              <td className="p-3">Sub-slice with negative index support.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">sum(arr)</td>
              <td className="p-3 font-mono">int | float</td>
              <td className="p-3">Sum of all numeric elements.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <CodeBlock
        language="persica"
        filename="helpers_demo.persi"
        code={`int[] data = [42, 10, 88, 3, 27];

print("Original:", data);
print("Sorted copy:", sort(data));
print("Reversed copy:", reverse(data));
print("Sum of elements:", sum(data));
print("Slice [1:4]:", slice(data, 1, 4));`}
        output={`Original: [42, 10, 88, 3, 27]
Sorted copy: [3, 10, 27, 42, 88]
Reversed copy: [27, 3, 88, 10, 42]
Sum of elements: 170
Slice [1:4]: [10, 88, 3]`}
      />

      <h2 id="dict" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Dictionaries (`dict`)
      </h2>
      <p>
        For flexible, JSON-like key-value data with mixed value types, use <code className="text-emerald-400">dict</code>:
      </p>

      <CodeBlock
        language="persica"
        filename="dictionaries.persi"
        code={`dict user = {
    "id": 101,
    "name": "Mohammad Taha",
    "verified": true,
    "rating": 4.9
};

print("User Name:", user["name"]);
user["role"] = "Admin";
print("User updated:", user);`}
        output={`User Name: Mohammad Taha
User updated: {"id": 101, "name": "Mohammad Taha", "verified": true, "rating": 4.9, "role": "Admin"}`}
      />

      <h2 id="maps" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Typed Maps (map&lt;K, V&gt;)
      </h2>
      <p>
        When you need strict static enforcement on key and value types, use <code className="text-emerald-400">map&lt;KeyType, ValueType&gt;</code>:
      </p>

      <CodeBlock
        language="persica"
        filename="typed_map.persi"
        code={`map<string, int> scores = {
    "Alice": 95,
    "Bob": 88
};

scores["Charlie"] = 92;
print("Alice's score:", scores["Alice"]);`}
        output={`Alice's score: 95`}
      />

      <h2 id="inspection" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Keys, Values & Inspection
      </h2>
      <CodeBlock
        language="persica"
        filename="inspection.persi"
        code={`dict config = {"host": "127.0.0.1", "port": 8080};

if contains_key(config, "port") {
    print("Port is configured:", config["port"]);
}

print("All keys:", keys(config));
print("All values:", values(config));`}
        output={`Port is configured: 8080
All keys: ["host", "port"]
All values: ["127.0.0.1", 8080]`}
      />
    </DocsLayout>
  );
}
