// @vitest-environment jsdom
import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ReportSection } from "./ReportSection";

describe("ReportSection", () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("renders read-only markdown output and fallback placeholder", () => {
    const { rerender } = render(<ReportSection reportMd="" />);

    const emptyTextarea = screen.getByPlaceholderText("Generate the report to view Markdown output.");
    expect(emptyTextarea).toHaveAttribute("readonly");
    expect(screen.getByText("EMPTY")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Download Markdown" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Print / Save PDF" })).toBeDisabled();

    rerender(<ReportSection reportMd="# Quarterly Incident Review\n\n- Summary" />);
    const renderedTextarea = screen.getByPlaceholderText("Generate the report to view Markdown output.");
    expect((renderedTextarea as HTMLTextAreaElement).value).toContain("Quarterly Incident Review");
    expect((renderedTextarea as HTMLTextAreaElement).value).toContain("- Summary");
    expect(screen.getByText("READY")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Download Markdown" })).toBeEnabled();
    expect(screen.getByRole("button", { name: "Print / Save PDF" })).toBeEnabled();
  });

  it("supports markdown download and browser PDF print output", () => {
    Object.defineProperty(URL, "createObjectURL", { value: vi.fn(), configurable: true });
    Object.defineProperty(URL, "revokeObjectURL", { value: vi.fn(), configurable: true });
    const createObjectURL = vi.spyOn(URL, "createObjectURL").mockReturnValue("blob:report");
    const revokeObjectURL = vi.spyOn(URL, "revokeObjectURL").mockImplementation(() => undefined);
    const anchorClick = vi.spyOn(HTMLAnchorElement.prototype, "click").mockImplementation(() => undefined);
    const print = vi.spyOn(window, "print").mockImplementation(() => undefined);

    render(<ReportSection reportMd="# Quarterly Incident Review\n\n- Summary" />);

    fireEvent.click(screen.getByRole("button", { name: "Download Markdown" }));
    fireEvent.click(screen.getByRole("button", { name: "Print / Save PDF" }));

    expect(createObjectURL).toHaveBeenCalledWith(expect.any(Blob));
    expect(anchorClick).toHaveBeenCalled();
    expect(revokeObjectURL).toHaveBeenCalledWith("blob:report");
    expect(print).toHaveBeenCalled();
  });
});
