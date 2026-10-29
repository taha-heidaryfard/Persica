import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';

export default function SyntaxBasicsPage() {
  const meta = {
    title: 'Variables & Static Data Types',
    category: 'Core Language',
    description: 'Learn about Persica’s static type system, primitive types, type inference, casting, and immutable final bindings.',
  };

  const toc = [
    { title: 'Primitive Types', id: 'primitives', level: 2 },
    { title: 'Variable Declarations', id: 'declarations', level: 2 },
    { title: 'Immutable Bindings with `final`', id: 'final', level: 2 },
    { title: 'Explicit Type Conversion', id: 'conversions', level: 2 },
    { title: 'Arithmetic & Logical Operators', id: 'operators', level: 2 },
    { title: 'Scope & Variable Shadowing', id: 'shadowing', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="primitives" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Primitive Data Types
      </h2>
      <p>
        Persica features a strong, static type system that checks all types at compile time before execution:
      </p>

      <div className="overflow-x-auto my-4">
        <table className="w-full text-left text-xs border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">Type</th>
              <th className="p-3">Description</th>
              <th className="p-3">Example Literals</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400">int</td>
              <td className="p-3">64-bit signed integer</td>
              <td className="p-3 font-mono">0, 42, -100</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">float</td>
              <td className="p-3">64-bit floating point number</td>
              <td className="p-3 font-mono">3.14159, -0.05, 1e6</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">string</td>
              <td className="p-3">Unicode UTF-8 string with escape sequences</td>
              <td className="p-3 font-mono">"Hello", "Persica 🌿", "\n"</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">bool</td>
              <td className="p-3">Boolean logical value</td>
              <td className="p-3 font-mono">true, false</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">void</td>
              <td className="p-3">Absence of value (for functions without return)</td>
              <td className="p-3 font-mono">-</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">any</td>
              <td className="p-3">Dynamic type allowing mixed values when needed</td>
              <td className="p-3 font-mono">-</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2 id="declarations" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Variable Declarations
      </h2>
      <p>
        Variables are declared by specifying the type followed by the identifier and an initial value:
      </p>

      <CodeBlock
        language="persica"
        filename="variables.persi"
        code={`int count = 10;
float pi = 3.14159265;
string language = "Persica";
bool is_production = true;

// Reassignment is allowed for standard variables
count = 20;
print("Current count:", count);`}
        output={`Current count: 20`}
      />

      <h2 id="final" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Immutable Bindings with `final`
      </h2>
      <p>
        To prevent accidental modification, prefix any declaration with the <code className="text-emerald-400 font-bold">final</code> keyword. The compiler enforces that final bindings cannot be reassigned:
      </p>

      <CodeBlock
        language="persica"
        filename="immutable.persi"
        code={`final string API_URL = "https://api.persica-lang.org/v1";
final int MAX_RETRIES = 5;

// The following line would trigger a compile-time static error:
// API_URL = "https://other.org"; // Error: cannot reassign to final variable 'API_URL'

print("API configured at:", API_URL);`}
        output={`API configured at: https://api.persica-lang.org/v1`}
      />

      <h2 id="conversions" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Explicit Type Conversion
      </h2>
      <p>
        Persica prevents subtle bugs by forbidding implicit type coercion between numeric types. All conversions are explicit:
      </p>

      <CodeBlock
        language="persica"
        filename="casting.persi"
        code={`// Convert string to integer or float
int parsed_int = to_int("450");
float parsed_float = to_float("12.75");

// Convert int to float and vice versa
float ratio = to_float(450) / 100.0;
int rounded = to_int(12.75); // truncates towards zero

// Convert any value to string
string text = to_string(parsed_int) + " items";
print(text, "with ratio", ratio);`}
        output={`450 items with ratio 4.5`}
      />

      <h2 id="operators" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Arithmetic & Logical Operators
      </h2>
      <p>
        Persica supports standard C/Python style operators with real short-circuiting:
      </p>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 my-4 text-xs">
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <h4 className="font-bold text-emerald-400 mb-2">Arithmetic & Compound:</h4>
          <ul className="space-y-1 text-slate-300 font-mono">
            <li>+ &nbsp;- &nbsp;* &nbsp;/ &nbsp;%</li>
            <li>+= &nbsp;-= &nbsp;*= &nbsp;/= &nbsp;%=</li>
            <li>++ &nbsp;-- &nbsp;(prefix/postfix)</li>
          </ul>
        </div>
        <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60">
          <h4 className="font-bold text-emerald-400 mb-2">Comparison & Logical:</h4>
          <ul className="space-y-1 text-slate-300 font-mono">
            <li>== &nbsp;!= &nbsp;&lt; &nbsp;&gt; &nbsp;&lt;= &nbsp;&gt;=</li>
            <li>&& &nbsp;|| &nbsp;! &nbsp;(short-circuit)</li>
            <li>and, or, not (supported keywords)</li>
          </ul>
        </div>
      </div>

      <h2 id="shadowing" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Scope & Variable Shadowing
      </h2>
      <p>
        Persica features strict lexical block scoping. Inner blocks can safely shadow outer variables without overwriting them:
      </p>

      <CodeBlock
        language="persica"
        filename="scoping.persi"
        code={`int value = 100;

{
    // Inner block shadows 'value'
    int value = 999;
    print("Inside block:", value);
}

print("Outside block:", value);`}
        output={`Inside block: 999
Outside block: 100`}
      />
    </DocsLayout>
  );
}
