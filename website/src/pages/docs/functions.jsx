import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';

export default function FunctionsPage() {
  const meta = {
    title: 'Functions & Recursion',
    category: 'Core Language',
    description: 'Master function definitions, static parameter typing, return validation, forward declarations, and recursive algorithms in Persica.',
  };

  const toc = [
    { title: 'Declaring Functions', id: 'declaring', level: 2 },
    { title: 'Void Functions', id: 'void-fns', level: 2 },
    { title: 'Static Return Path Validation', id: 'return-paths', level: 2 },
    { title: 'Recursion & Forward Calls', id: 'recursion', level: 2 },
    { title: 'Shadowing Functions', id: 'shadowing', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="declaring" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Declaring Functions
      </h2>
      <p>
        Functions in Persica are statically typed. You declare the return type, the function name, and typed parameters enclosed in parentheses:
      </p>

      <CodeBlock
        language="persica"
        filename="functions.persi"
        code={`// Typed function accepting two integers and returning an integer
def int multiply(int a, int b) {
    return a * b;
}

// Float function computing circle area
def float circle_area(float radius) {
    final float PI = 3.14159265;
    return PI * radius * radius;
}

print("5 * 8 =", multiply(5, 8));
print("Area of radius 4.0 =", circle_area(4.0));`}
        output={`5 * 8 = 40
Area of radius 4.0 = 50.2654824`}
      />

      <h2 id="void-fns" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Void Functions
      </h2>
      <p>
        Functions that perform actions (like logging or updating external state) without returning a value declare <code className="text-emerald-400">void</code> as their return type. Early return is possible using a standalone <code className="text-emerald-400">return;</code> statement:
      </p>

      <CodeBlock
        language="persica"
        filename="void_functions.persi"
        code={`def void log_warning(string message, bool is_critical) {
    if !is_critical {
        print("[INFO]", message);
        return; // early return
    }
    print("[CRITICAL ALERT]", message);
}

log_warning("Cache refreshed", false);
log_warning("Database connection timeout", true);`}
        output={`[INFO] Cache refreshed
[CRITICAL ALERT] Database connection timeout`}
      />

      <h2 id="return-paths" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Static Return Path Validation
      </h2>
      <p>
        Persica’s semantic analyzer verifies that <strong>every possible branch</strong> of a typed function produces a return value. If an <code className="text-emerald-300">if/else</code> tree fails to cover a path, the compiler reports a static compile-time error:
      </p>

      <CodeBlock
        language="persica"
        filename="return_safety.persi"
        code={`// Valid: All execution paths return an integer
def int absolute_value(int n) {
    if n < 0 {
        return -n;
    } else {
        return n;
    }
}

// Valid: 'throw' satisfies the return requirement
def string get_status(int code) {
    if code == 200 {
        return "OK";
    }
    throw "Invalid status code";
}`}
      />

      <h2 id="recursion" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Recursion & Forward Calls
      </h2>
      <p>
        Functions support recursion and forward references seamlessly. You can call a function defined later in the file without requiring forward prototypes:
      </p>

      <CodeBlock
        language="persica"
        filename="recursion.persi"
        code={`// Recursive Factorial
def int factorial(int n) {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

// Mutual recursion: is_even calls is_odd, which is defined below
def bool is_even(int n) {
    if n == 0 { return true; }
    return is_odd(n - 1);
}

def bool is_odd(int n) {
    if n == 0 { return false; }
    return is_even(n - 1);
}

print("5! =", factorial(5));
print("Is 14 even?", is_even(14));`}
        output={`5! = 120
Is 14 even? true`}
      />
    </DocsLayout>
  );
}
