import React from 'react';
import DocsLayout from '../../components/DocsLayout';
import CodeBlock from '../../components/CodeBlock';
import { FolderTree, Package, CheckCircle2, ArrowRight } from 'lucide-react';

export default function ProjectSystemPage() {
  const meta = {
    title: 'Project Architecture & .persi_env',
    category: 'Project System',
    badge: 'New in v0.13.0',
    description: 'Structure multi-file Persica projects, manage modules, configure persica.toml, and use isolated local package environments.',
  };

  const toc = [
    { title: 'Overview & Philosophy', id: 'overview', level: 2 },
    { title: 'Scaffolding with `persica new`', id: 'new-command', level: 2 },
    { title: 'The `persica.toml` Manifest', id: 'toml-schema', level: 2 },
    { title: 'The Isolated `.persi_env` Environment', id: 'persi-env', level: 2 },
    { title: 'Module Resolution Hierarchy', id: 'resolution', level: 2 },
    { title: 'CLI Project Commands', id: 'commands', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="overview" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        Overview & Philosophy
      </h2>
      <p>
        Introduced in <strong>v0.13.0</strong>, Persica features a built-in, zero-dependency project system. It enables developers to structure large-scale codebases cleanly with modular imports, project-level unit testing, and isolated package environments.
      </p>

      <h2 id="new-command" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Scaffolding a New Project
      </h2>
      <p>
        To create a new Persica project with complete scaffolding in one command, run:
      </p>

      <CodeBlock
        language="bash"
        filename="Terminal"
        code={`persica new analytics_app
cd analytics_app`}
        output={`Created new Persica project 'analytics_app'
  - Created persica.toml
  - Created src/main.persi
  - Created .persi_env/ environment directory`}
      />

      <p className="text-xs font-semibold text-slate-300">Generated Project Layout:</p>
      <div className="my-3 p-4 rounded-xl border border-slate-800 bg-slate-900/60 font-mono text-xs text-slate-300">
        <div>📁 analytics_app/</div>
        <div className="pl-4">├── 📄 persica.toml &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// Project manifest configuration</span></div>
        <div className="pl-4">├── 📁 src/ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// Application source code</span></div>
        <div className="pl-8">└── 📄 main.persi &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// Main application entrypoint</span></div>
        <div className="pl-4">└── 📁 .persi_env/ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span className="text-slate-500">// Isolated local dependencies folder</span></div>
      </div>

      <h2 id="toml-schema" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        The `persica.toml` Manifest
      </h2>
      <p>
        The <code className="text-emerald-400">persica.toml</code> file is the root configuration of your package. It uses a clean, zero-dependency TOML structure:
      </p>

      <CodeBlock
        language="toml"
        filename="persica.toml"
        code={`[package]
name = "analytics_app"
version = "0.1.0"
entry = "src/main.persi"
source = "src"

[environment]
env_dir = ".persi_env"`}
      />

      <div className="overflow-x-auto my-4">
        <table className="w-full text-left text-xs border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">Field</th>
              <th className="p-3">Default</th>
              <th className="p-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400">package.name</td>
              <td className="p-3 font-mono">""</td>
              <td className="p-3">Unique identifier for the project.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">package.version</td>
              <td className="p-3 font-mono">"0.1.0"</td>
              <td className="p-3">Semantic version number of the package.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">package.entry</td>
              <td className="p-3 font-mono">"src/main.persi"</td>
              <td className="p-3">File executed when running <code>persica run</code> or <code>persica persica.toml</code>.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">package.source</td>
              <td className="p-3 font-mono">"src"</td>
              <td className="p-3">Root directory for internal module resolution.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400">environment.env_dir</td>
              <td className="p-3 font-mono">".persi_env"</td>
              <td className="p-3">Dedicated folder for isolated local modules and external packages.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2 id="persi-env" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        The Isolated `.persi_env` Environment
      </h2>
      <p>
        In modern programming, isolating project dependencies is critical. In Python, this is done via virtual environments (<code className="text-slate-400">.venv</code>), but virtualenvs duplicate Python executable binaries and dynamic link libraries.
      </p>
      <div className="p-4 rounded-xl border border-emerald-500/30 bg-emerald-500/10 text-xs text-slate-300 space-y-2 my-4">
        <h4 className="font-bold text-emerald-400 text-sm">Persica’s Clean Design:</h4>
        <p>
          Persica’s <code className="text-emerald-300 font-bold">.persi_env</code> is completely configuration-driven. <strong>Zero executable binaries are duplicated.</strong> The compiler automatically links your local environment into the module resolution path.
        </p>
      </div>

      <h2 id="resolution" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        Module Resolution Hierarchy
      </h2>
      <p>
        When you write an import statement such as <code className="text-emerald-400">from my_library import calculate;</code>, the compiler searches through four tiered layers in strict order:
      </p>

      <ol className="my-4 space-y-3 text-xs text-slate-300 list-decimal pl-5">
        <li>
          <strong>Current File Directory:</strong> Relative to the invoking script (<code className="text-emerald-400">./my_library.persi</code> or <code className="text-emerald-400">./my_library/mod.persi</code>).
        </li>
        <li>
          <strong>Project Source Directory:</strong> Configured as <code className="text-emerald-400">source</code> in <code className="text-emerald-400">persica.toml</code> (e.g. <code className="text-emerald-400">src/my_library.persi</code>).
        </li>
        <li>
          <strong>Local Environment (.persi_env):</strong> Configured as <code className="text-emerald-400">env_dir</code> (e.g. <code className="text-emerald-400">.persi_env/my_library/mod.persi</code>).
        </li>
        <li>
          <strong>Built-in Standard Library:</strong> The self-hosted stdlib (<code className="text-emerald-400">std.math</code>, <code className="text-emerald-400">std.collections</code>, etc.).
        </li>
      </ol>

      <h2 id="commands" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        CLI Project Commands
      </h2>

      <div className="space-y-6 my-4">
        <div>
          <h4 className="text-sm font-bold text-white">Running the Project:</h4>
          <CodeBlock
            language="bash"
            filename="Terminal"
            code={`persica run`}
            output={`Running project 'analytics_app' (v0.1.0)...
[Application output]`}
          />
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">Running Automated Project Tests:</h4>
          <p className="text-xs text-slate-400 mb-2">
            Discovers and executes every <code className="text-emerald-300">*.persi</code> test file in the <code className="text-emerald-300">tests/</code> directory:
          </p>
          <CodeBlock
            language="bash"
            filename="Terminal"
            code={`persica test`}
            output={`Running 3 test file(s) in 'tests' directory:
  - tests/math_test.persi ... PASSED
  - tests/model_test.persi ... PASSED
  - tests/api_test.persi ... PASSED

Summary: 3 passed, 0 failed`}
          />
        </div>

        <div>
          <h4 className="text-sm font-bold text-white">Direct Manifest Invocation:</h4>
          <p className="text-xs text-slate-400 mb-2">
            You can explicitly point Persica to a configuration and optionally specify an alternate script to run within that project context:
          </p>
          <CodeBlock
            language="bash"
            filename="Terminal"
            code={`# Run the entry file defined in persica.toml
persica persica.toml

# Run an alternate script with the project paths loaded
persica persica.toml src/benchmarks/perf.persi`}
          />
        </div>
      </div>
    </DocsLayout>
  );
}
