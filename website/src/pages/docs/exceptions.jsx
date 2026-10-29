import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';
import { ShieldCheck, AlertTriangle } from 'lucide-react';

export default function ExceptionsPage() {
  const meta = {
    title: 'Exception Handling (try / catch / throw)',
    category: 'Object-Oriented & Robustness',
    description: 'Learn how Persica prevents panics and manages errors gracefully using try, catch, and throw with complete call-stack unwinding.',
  };

  const toc = [
    { title: 'The Zero-Panic Guarantee', id: 'guarantee', level: 2 },
    { title: 'Basic `try` and `catch`', id: 'basic', level: 2 },
    { title: 'The `throw` Statement', id: 'throw', level: 2 },
    { title: 'Catch Syntax Variants', id: 'variants', level: 2 },
    { title: 'Trapping Built-in Runtime Errors', id: 'runtime-trapping', level: 2 },
    { title: 'Call-Stack Unwinding', id: 'unwinding', level: 2 },
    { title: 'Re-throwing Exceptions', id: 'rethrow', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="guarantee" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        The Zero-Panic Guarantee
      </h2>
      <p>
        In Persica, <strong>no error in user code is allowed to crash or panic the interpreter</strong>. The language provides a comprehensive exception handling model allowing programs to intercept both custom user errors and internal runtime errors (like division by zero or array bounds violations) safely.
      </p>

      <h2 id="basic" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Basic `try` and `catch`
      </h2>
      <p>
        Wrap vulnerable code blocks inside a <code className="text-emerald-400 font-bold">try</code> block followed by a <code className="text-emerald-400 font-bold">catch</code> block:
      </p>

      <CodeBlock
        language="persica"
        filename="basic_try.persi"
        code={`try {
    print("Executing sensitive operation...");
    throw "Database unreachable!";
    print("This line will never be reached.");
} catch (error) {
    print("Intercepted error message:", error);
}

print("Application recovered smoothly! ✨");`}
        output={`Executing sensitive operation...
Intercepted error message: Database unreachable!
Application recovered smoothly! ✨`}
      />

      <h2 id="throw" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        The `throw` Statement
      </h2>
      <p>
        You can throw any expression: descriptive error strings, numeric HTTP status codes, structured dictionaries, or custom objects:
      </p>

      <CodeBlock
        language="persica"
        filename="throwing.persi"
        code={`// Throwing numeric error codes
try {
    throw 404;
} catch (code) {
    print("Caught HTTP Error Code:", code);
}

// Throwing structured dictionary payloads
try {
    throw {
        "status": 500,
        "detail": "Failed to serialize JSON payload"
    };
} catch (err_dict) {
    print("Status:", err_dict["status"]);
    print("Detail:", err_dict["detail"]);
}`}
        output={`Caught HTTP Error Code: 404
Status: 500
Detail: Failed to serialize JSON payload`}
      />

      <h2 id="variants" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Catch Syntax Variants
      </h2>
      <p>
        Persica’s parser supports several flexible styles for catching exceptions:
      </p>

      <CodeBlock
        language="persica"
        filename="syntax_styles.persi"
        code={`// 1. Standard with parentheses
try { throw "err1"; } catch (e) { print(e); }

// 2. Pythonic without parentheses
try { throw "err2"; } catch e { print(e); }

// 3. Explicit type annotation
try { throw "err3"; } catch (string e) { print(e); }

// 4. Anonymous catch (discard error value)
try { throw "err4"; } catch { print("Error caught silently"); }`}
      />

      <h2 id="runtime-trapping" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Trapping Built-in Runtime Errors
      </h2>
      <p>
        Built-in operational faults (e.g. division by zero, missing dictionary keys, array index out of bounds) are automatically converted into catchable exceptions:
      </p>

      <CodeBlock
        language="persica"
        filename="trapping.persi"
        code={`// Trapping division by zero
try {
    int a = 10;
    int b = 0;
    int c = a / b;
} catch (err) {
    print("Trapped arithmetic error:", err);
}

// Trapping array bounds violation
try {
    int[] list = [1, 2, 3];
    int invalid = list[99];
} catch (err) {
    print("Trapped bounds error:", err);
}`}
        output={`Trapped arithmetic error: runtime error: division by zero
Trapped bounds error: runtime error: array index out of bounds: index 99 but length is 3`}
      />

      <h2 id="unwinding" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Call-Stack Unwinding
      </h2>
      <p>
        When an exception occurs deep inside nested function calls, Persica cleanly unwinds the call stack and releases scoped memory until it reaches an active enclosing <code className="text-emerald-300">try</code> block:
      </p>

      <CodeBlock
        language="persica"
        filename="stack_unwinding.persi"
        code={`void level3() {
    print("In level 3: about to throw");
    throw "Fatal failure in level 3!";
    print("Unreachable");
}

void level2() {
    print("In level 2");
    level3();
}

void level1() {
    print("In level 1");
    level2();
}

try {
    level1();
} catch (failure) {
    print("Caught at top-level:", failure);
}`}
        output={`In level 1
In level 2
In level 3: about to throw
Caught at top-level: Fatal failure in level 3!`}
      />

      <h2 id="rethrow" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Re-throwing Exceptions
      </h2>
      <p>
        Catch blocks can log or audit an error, and then re-throw it to outer layers:
      </p>

      <CodeBlock
        language="persica"
        filename="rethrow.persi"
        code={`try {
    try {
        throw "Initial trigger";
    } catch (inner) {
        print("Logged in inner handler:", inner);
        throw inner; // Re-throw to outer block
    }
} catch (outer) {
    print("Handled finally in outer block:", outer);
}`}
        output={`Logged in inner handler: Initial trigger
Handled finally in outer block: Initial trigger`}
      />
    </DocsLayout>
  );
}
