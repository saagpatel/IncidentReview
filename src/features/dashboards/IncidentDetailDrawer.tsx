import { formatSeconds } from "../../lib/format";

export type IncidentDetailPayload = {
  incident: {
    id: number;
    external_id: string | null;
    fingerprint: string;
    title: string;
    description: string | null;
    severity: string | null;
    detection_source: string | null;
    vendor: string | null;
    service: string | null;
    impact_pct: number | null;
    service_health_pct: number | null;
    start_ts: string | null;
    first_observed_ts: string | null;
    it_awareness_ts: string | null;
    ack_ts: string | null;
    mitigate_ts: string | null;
    resolve_ts: string | null;
    start_ts_raw: string | null;
    first_observed_ts_raw: string | null;
    it_awareness_ts_raw: string | null;
    ack_ts_raw: string | null;
    mitigate_ts_raw: string | null;
    resolve_ts_raw: string | null;
  };
  metrics: {
    mttd_seconds: number | null;
    it_awareness_lag_seconds: number | null;
    mtta_seconds: number | null;
    time_to_mitigation_seconds: number | null;
    mttr_seconds: number | null;
  };
  warnings: Array<{ code: string; message: string; details?: string | null }>;
  artifacts: Array<{
    id: number;
    incident_id: number | null;
    kind: string;
    sha256: string;
    filename: string | null;
    mime_type: string | null;
    text: string | null;
    created_at: string;
  }>;
  timeline_events: Array<{
    id: number;
    incident_id: number | null;
    source: string;
    ts: string | null;
    author: string | null;
    kind: string | null;
    text: string;
    raw_json: string | null;
    created_at: string;
  }>;
};

export function IncidentDetailDrawer(props: {
  open: boolean;
  loading: boolean;
  detail: IncidentDetailPayload | null;
  onClose: () => void;
}) {
  if (!props.open) return null;
  const incident = props.detail?.incident;

  return (
    <div
      className="drawerOverlay"
      onClick={(e) => {
        if (e.target === e.currentTarget) props.onClose();
      }}
      onKeyDown={(e) => {
        if (e.key === "Escape" || e.key === "Enter" || e.key === " ") props.onClose();
      }}
      role="button"
      tabIndex={0}
      aria-label="Close incident detail drawer"
    >
      <aside className="drawer" role="dialog" aria-modal="true">
        <div className="drawerHeader">
          <div>
            <div className="muted">Incident detail</div>
            <div className="drawerTitle">
              {props.detail?.incident.external_id ?? "NO_EXTERNAL_ID"}:{" "}
              {props.detail?.incident.title ?? (props.loading ? "Loading..." : "")}
            </div>
          </div>
          <button className="btn" type="button" onClick={props.onClose}>
            Close
          </button>
        </div>

        {props.loading && <p className="muted">Loading incident detail...</p>}

        {props.detail && (
          <div className="drawerBody">
            <section className="drawerSection">
              <h3>Incident context</h3>
              <dl className="drawerMeta">
                <div>
                  <dt>Severity</dt>
                  <dd>{incident?.severity ?? "UNKNOWN"}</dd>
                </div>
                <div>
                  <dt>Detection source</dt>
                  <dd>{incident?.detection_source ?? "UNKNOWN"}</dd>
                </div>
                <div>
                  <dt>Vendor</dt>
                  <dd>{incident?.vendor ?? "UNKNOWN"}</dd>
                </div>
                <div>
                  <dt>Service</dt>
                  <dd>{incident?.service ?? "UNKNOWN"}</dd>
                </div>
                <div>
                  <dt>Impact</dt>
                  <dd>{formatPercent(incident?.impact_pct)}</dd>
                </div>
                <div>
                  <dt>Service health</dt>
                  <dd>{formatPercent(incident?.service_health_pct)}</dd>
                </div>
                <div>
                  <dt>Fingerprint</dt>
                  <dd className="mono">{incident?.fingerprint ?? "UNKNOWN"}</dd>
                </div>
              </dl>
            </section>

            <section className="drawerSection">
              <h3>Computed metrics (deterministic)</h3>
              <ul className="list">
                <li>
                  <span className="mono">MTTD</span>: {formatSeconds(props.detail.metrics.mttd_seconds)}
                </li>
                <li>
                  <span className="mono">Awareness lag</span>: {formatSeconds(props.detail.metrics.it_awareness_lag_seconds)}
                </li>
                <li>
                  <span className="mono">MTTA</span>: {formatSeconds(props.detail.metrics.mtta_seconds)}
                </li>
                <li>
                  <span className="mono">Time to mitigate</span>: {formatSeconds(props.detail.metrics.time_to_mitigation_seconds)}
                </li>
                <li>
                  <span className="mono">MTTR</span>: {formatSeconds(props.detail.metrics.mttr_seconds)}
                </li>
              </ul>
            </section>

            <section className="drawerSection">
              <h3>Validation/anomalies</h3>
              {props.detail.warnings.length === 0 ? (
                <p className="muted">No warnings.</p>
              ) : (
                <ul className="list">
                  {props.detail.warnings.map((w, idx) => (
                    <li key={idx}>
                      <span className="mono">{w.code}</span>: {w.message}{" "}
                      {w.details ? <span className="mono">({w.details})</span> : null}
                    </li>
                  ))}
                </ul>
              )}
            </section>

            <section className="drawerSection">
              <h3>Timestamp evidence</h3>
              <dl className="drawerMeta">
                <TimestampEvidence label="Started" value={incident?.start_ts} raw={incident?.start_ts_raw} />
                <TimestampEvidence label="First observed" value={incident?.first_observed_ts} raw={incident?.first_observed_ts_raw} />
                <TimestampEvidence label="IT aware" value={incident?.it_awareness_ts} raw={incident?.it_awareness_ts_raw} />
                <TimestampEvidence label="Acknowledged" value={incident?.ack_ts} raw={incident?.ack_ts_raw} />
                <TimestampEvidence label="Mitigated" value={incident?.mitigate_ts} raw={incident?.mitigate_ts_raw} />
                <TimestampEvidence label="Resolved" value={incident?.resolve_ts} raw={incident?.resolve_ts_raw} />
              </dl>
            </section>

            <section className="drawerSection">
              <h3>Timeline events</h3>
              {props.detail.timeline_events.length === 0 ? (
                <p className="muted">No timeline events attached.</p>
              ) : (
                <ul className="list">
                  {props.detail.timeline_events.map((e) => (
                    <li key={e.id}>
                      {e.text === "[REDACTED]" || (e.raw_json && e.raw_json.includes('"text_redacted":true')) ? (
                        <span className="muted">(redacted)</span>
                      ) : null}{" "}
                      <span className="mono">{e.ts ?? "UNKNOWN_TS"}</span> <span className="muted">({e.source})</span>: {e.text}
                    </li>
                  ))}
                </ul>
              )}
            </section>

            <section className="drawerSection">
              <h3>Artifacts</h3>
              {props.detail.artifacts.length === 0 ? (
                <p className="muted">No artifacts attached.</p>
              ) : (
                <ul className="list">
                  {props.detail.artifacts.map((a) => (
                    <li key={a.id}>
                      <span className="mono">{a.kind}</span> {a.filename ? <span className="mono">({a.filename})</span> : null}{" "}
                      <span className="muted">{a.sha256.slice(0, 12)}...</span>
                    </li>
                  ))}
                </ul>
              )}
            </section>
          </div>
        )}
      </aside>
    </div>
  );
}

function formatPercent(value: number | null | undefined): string {
  if (value == null) return "UNKNOWN";
  return `${value}%`;
}

function TimestampEvidence(props: { label: string; value?: string | null; raw?: string | null }) {
  const display = props.value ?? props.raw ?? "UNKNOWN";
  const suffix = props.value ? "canonical" : props.raw ? "raw" : "missing";
  return (
    <div>
      <dt>{props.label}</dt>
      <dd>
        <span className="mono">{display}</span> <span className="muted">({suffix})</span>
      </dd>
    </div>
  );
}
