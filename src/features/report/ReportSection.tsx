export function ReportSection(props: { reportMd: string }) {
  const hasReport = props.reportMd.trim().length > 0;
  const lineCount = hasReport ? props.reportMd.split(/\r?\n/).length : 0;
  const wordCount = hasReport ? props.reportMd.trim().split(/\s+/).length : 0;

  function onDownloadMarkdown() {
    if (!hasReport) return;
    const blob = new Blob([props.reportMd], { type: "text/markdown;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "quarterly-incident-review.md";
    a.click();
    URL.revokeObjectURL(url);
  }

  function onPrintPdf() {
    if (!hasReport) return;
    window.print();
  }

  return (
    <section className="card" id="report">
      <h2>QIR Report (Markdown)</h2>
      <div className="kpiRow">
        <div className="kpi">
          <div className="kpi__label">Output</div>
          <div className="kpi__value">{hasReport ? "READY" : "EMPTY"}</div>
        </div>
        <div className="kpi">
          <div className="kpi__label">Lines</div>
          <div className="kpi__value">{lineCount}</div>
        </div>
        <div className="kpi">
          <div className="kpi__label">Words</div>
          <div className="kpi__value">{wordCount}</div>
        </div>
      </div>
      <div className="actions">
        <button className="btn btn--accent" type="button" onClick={onDownloadMarkdown} disabled={!hasReport}>
          Download Markdown
        </button>
        <button className="btn" type="button" onClick={onPrintPdf} disabled={!hasReport}>
          Print / Save PDF
        </button>
      </div>
      <textarea className="md" value={props.reportMd} readOnly placeholder="Generate the report to view Markdown output." />
    </section>
  );
}
