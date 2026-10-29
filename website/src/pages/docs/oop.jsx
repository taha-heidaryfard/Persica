import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';

export default function OopPage() {
  const meta = {
    title: 'Classes & Structs (OOP)',
    category: 'Object-Oriented & Robustness',
    description: 'Learn Persica’s dual object model: pure-data value structs vs. reference-based object classes with methods and state.',
  };

  const toc = [
    { title: 'The Dual Object Philosophy', id: 'philosophy', level: 2 },
    { title: 'Pure Value Structs (`struct`)', id: 'structs', level: 2 },
    { title: 'Object-Oriented Classes (`class`)', id: 'classes', level: 2 },
    { title: 'Methods and the `this` Keyword', id: 'methods', level: 2 },
    { title: 'Value vs. Reference Semantics', id: 'semantics', level: 2 },
    { title: 'Collections of Objects', id: 'collections', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="philosophy" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        The Dual Object Philosophy
      </h2>
      <p>
        Persica solves one of the oldest debates in language design by providing two distinct, purpose-built constructs:
      </p>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 my-4 text-xs">
        <div className="p-4 rounded-xl border border-blue-500/30 bg-blue-500/10 space-y-1.5">
          <h4 className="font-bold text-blue-300 text-sm">struct (Pure Data)</h4>
          <p className="text-slate-300">Used for lightweight models: coordinates, dimensions, RGB colors, points.</p>
          <ul className="list-disc pl-4 space-y-0.5 text-slate-400">
            <li><strong>Value Semantics:</strong> Copied when passed or assigned.</li>
            <li><strong>No Methods:</strong> Pure state without behavior.</li>
            <li><strong>Equality:</strong> Deep structural comparison.</li>
          </ul>
        </div>

        <div className="p-4 rounded-xl border border-emerald-500/30 bg-emerald-500/10 space-y-1.5">
          <h4 className="font-bold text-emerald-300 text-sm">class (State & Behavior)</h4>
          <p className="text-slate-300">Used for active entities: bank accounts, database clients, network sockets.</p>
          <ul className="list-disc pl-4 space-y-0.5 text-slate-400">
            <li><strong>Reference Semantics:</strong> Shared reference when assigned.</li>
            <li><strong>Has Methods:</strong> Full access to internal state via <code className="text-emerald-300">this</code>.</li>
            <li><strong>Identity:</strong> Mutating one reference affects all shared holders.</li>
          </ul>
        </div>
      </div>

      <h2 id="structs" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Pure Value Structs (`struct`)
      </h2>
      <p>
        Define a struct by naming its fields. Instances are instantiated by calling the struct name like a function:
      </p>

      <CodeBlock
        language="persica"
        filename="struct_demo.persi"
        code={`struct Point {
    int x;
    int y;
}

Point p1 = Point(10, 20);
print("Point p1:", p1.x, p1.y);

// Copy by value: modifying p2 does NOT affect p1
Point p2 = p1;
p2.x = 99;

print("p1.x after p2 change:", p1.x);
print("p2.x:", p2.x);`}
        output={`Point p1: 10 20
p1.x after p2 change: 10
p2.x: 99`}
      />

      <h2 id="classes" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Object-Oriented Classes (`class`)
      </h2>
      <p>
        Classes encapsulate fields with optional default values and methods:
      </p>

      <CodeBlock
        language="persica"
        filename="class_demo.persi"
        code={`class StopWatch {
    int elapsed_seconds;
    bool is_running;

    def void start() {
        this.is_running = true;
    }

    def void tick(int seconds) {
        if this.is_running {
            this.elapsed_seconds += seconds;
        }
    }

    def int get_elapsed() {
        return this.elapsed_seconds;
    }
}

StopWatch timer = StopWatch(0, false);
timer.start();
timer.tick(15);
timer.tick(30);
print("Elapsed time:", timer.get_elapsed(), "seconds");`}
        output={`Elapsed time: 45 seconds`}
      />

      <h2 id="methods" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Methods & The `this` Keyword
      </h2>
      <p>
        Within any method, the <code className="text-emerald-400 font-bold">this</code> keyword explicitly references the active instance. Methods can call sibling methods directly:
      </p>

      <CodeBlock
        language="persica"
        filename="bank_account.persi"
        code={`class Account {
    string owner;
    int balance;

    def void deposit(int amount) {
        this.balance += amount;
        this.log_audit("Deposited " + to_string(amount));
    }

    def void log_audit(string event) {
        print("[AUDIT]", this.owner, "-", event, "| Balance:", this.balance);
    }
}

Account acc = Account("Mohammad Taha", 0);
acc.deposit(500);
acc.deposit(250);`}
        output={`[AUDIT] Mohammad Taha - Deposited 500 | Balance: 500
[AUDIT] Mohammad Taha - Deposited 250 | Balance: 750`}
      />

      <h2 id="semantics" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Value vs. Reference Semantics
      </h2>
      <p>
        When you assign an instance of a <code className="text-emerald-300 font-semibold">class</code> to another variable, both variables point to the same underlying heap object (managed with Rust’s reference counted cell):
      </p>

      <CodeBlock
        language="persica"
        filename="references.persi"
        code={`class SharedState {
    int counter;
}

SharedState ref1 = SharedState(0);
SharedState ref2 = ref1; // Shared reference

ref2.counter = 42;

// ref1 sees the change because both share the same instance!
print("ref1.counter:", ref1.counter);`}
        output={`ref1.counter: 42`}
      />

      <h2 id="collections" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Collections of Objects
      </h2>
      <p>
        Both structs and classes can be stored within arrays and maps:
      </p>

      <CodeBlock
        language="persica"
        filename="object_lists.persi"
        code={`struct Task {
    int id;
    string title;
    bool done;
}

Task[] todo_list = [
    Task(1, "Install Persica", true),
    Task(2, "Learn OOP syntax", true),
    Task(3, "Build awesome app", false)
];

for task in todo_list {
    string status = task.done ? "[DONE]" : "[PENDING]";
    print(status, task.title);
}`}
        output={`[DONE] Install Persica
[DONE] Learn OOP syntax
[PENDING] Build awesome app`}
      />
    </DocsLayout>
  );
}
