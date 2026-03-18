import { getDefaultFont, type Font, type Template } from "@pdfme/common";
import {
  text,
  multiVariableText,
  image,
  svg,
  table,
  barcodes,
  line,
  rectangle,
  ellipse,
} from "@pdfme/schemas";
import type { Plugin } from "@pdfme/common";
import type { ResultEntry } from "../api/api";

// ── URL Image plugin ──────────────────────────────────────────────────────────
// Stores an image URL as content instead of a base64 data URI.
// The image is fetched at PDF-generation time, so the template stays small.

const MM_TO_PT = 72 / 25.4;

async function urlToDataUrl(url: string): Promise<string> {
  const res = await fetch(url);
  const blob = await res.blob();
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = reject;
    reader.readAsDataURL(blob);
  });
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const urlImage: Plugin<any> = {
  pdf: async (arg) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const { value, schema, pdfDoc, page, _cache } = arg as any;
    const url: string = value || schema.content || "";
    if (!url) return;

    let embedded = _cache.get(url);
    if (!embedded) {
      const dataUrl = await urlToDataUrl(url);
      const isPng = dataUrl.startsWith("data:image/png;");
      embedded = await (isPng ? pdfDoc.embedPng(dataUrl) : pdfDoc.embedJpg(dataUrl));
      _cache.set(url, embedded);
    }

    const x      = schema.position.x * MM_TO_PT;
    const width  = schema.width       * MM_TO_PT;
    const height = schema.height      * MM_TO_PT;
    const y      = page.getHeight() - schema.position.y * MM_TO_PT - height;
    page.drawImage(embedded, { x, y, width, height, opacity: schema.opacity ?? 1 });
  },

  ui: (arg) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const { value, schema, rootElement, mode, onChange } = arg as any;
    const url: string = value || schema.content || "";
    const editable = mode === "designer";

    rootElement.style.cssText = "width:100%;height:100%;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:4px;box-sizing:border-box;";

    if (url) {
      const img = document.createElement("img");
      img.src = url;
      img.style.cssText = "max-width:100%;max-height:calc(100% - 28px);object-fit:contain;";
      rootElement.appendChild(img);
    }

    if (editable) {
      const input = document.createElement("input");
      input.type = "text";
      input.placeholder = "https://…";
      input.value = url;
      input.style.cssText = "width:100%;font-size:11px;padding:2px 4px;border:1px solid #ccc;border-radius:3px;box-sizing:border-box;";
      input.addEventListener("change", () => {
        if (onChange) onChange({ key: "content", value: input.value });
      });
      rootElement.appendChild(input);
    } else if (!url) {
      const hint = document.createElement("span");
      hint.textContent = "Kein Bild";
      hint.style.cssText = "font-size:11px;color:#999;";
      rootElement.appendChild(hint);
    }
  },

  propPanel: {
    schema: {},
    defaultSchema: {
      name: "",
      type: "urlImage",
      content: "",
      position: { x: 0, y: 0 },
      width:  40,
      height: 40,
      opacity: 1,
    },
  },
};

// ── Fonts ────────────────────────────────────────────────────────────────────

export function getFontsData(): Font {
  return {
    ...getDefaultFont(),
    // ── Script / calligraphy ──────────────────────────────────────────────────
    "PinyonScript-Regular": {      // OFL – elegant script
      fallback: false,
      data: "https://fonts.gstatic.com/s/pinyonscript/v22/6xKpdSJbL9-e9LuoeQiDRQR8aOLQO4bhiDY.ttf",
    },
    "GreatVibes-Regular": {        // OFL – flowing script, great for certificates
      fallback: false,
      data: "https://fonts.gstatic.com/s/greatvibes/v21/RWmMoKWR9v4ksMfaWd_JN-XC.ttf",
    },
    // ── Formal serif ─────────────────────────────────────────────────────────
    "Cinzel-Regular": {            // OFL – classical Roman caps, ideal for formal docs
      fallback: false,
      data: "https://fonts.gstatic.com/s/cinzel/v26/8vIU7ww63mVu7gtR-kwKxNvkNOjw-tbnTYo.ttf",
    },
    "Cinzel-Bold": {               // OFL
      fallback: false,
      data: "https://fonts.gstatic.com/s/cinzel/v26/8vIU7ww63mVu7gtR-kwKxNvkNOjw-jHgTYo.ttf",
    },
    "PlayfairDisplay-Regular": {   // OFL – elegant serif for headlines
      fallback: false,
      data: "https://fonts.gstatic.com/s/playfairdisplay/v40/nuFvD-vYSZviVYUb_rj3ij__anPXJzDwcbmjWBN2PKdFvUDQ.ttf",
    },
    "PlayfairDisplay-Bold": {      // OFL
      fallback: false,
      data: "https://fonts.gstatic.com/s/playfairdisplay/v40/nuFvD-vYSZviVYUb_rj3ij__anPXJzDwcbmjWBN2PKeiukDQ.ttf",
    },
    "Merriweather-Regular": {      // OFL – highly legible serif body text
      fallback: false,
      data: "https://fonts.gstatic.com/s/merriweather/v33/u-4D0qyriQwlOrhSvowK_l5UcA6zuSYEqOzpPe3HOZJ5eX1WtLaQwmYiScCmDxhtNOKl8yDr3icqEw.ttf",
    },
    "Merriweather-Bold": {         // OFL
      fallback: false,
      data: "https://fonts.gstatic.com/s/merriweather/v33/u-4D0qyriQwlOrhSvowK_l5UcA6zuSYEqOzpPe3HOZJ5eX1WtLaQwmYiScCmDxhtNOKl8yDrOSAqEw.ttf",
    },
    // ── Sans-serif ────────────────────────────────────────────────────────────
    "Roboto-Regular": {            // Apache 2.0 – Google's flagship sans-serif
      fallback: false,
      data: "https://fonts.gstatic.com/s/roboto/v51/KFOMCnqEu92Fr1ME7kSn66aGLdTylUAMQXC89YmC2DPNWubEbWmT.ttf",
    },
    "Roboto-Bold": {               // Apache 2.0
      fallback: false,
      data: "https://fonts.gstatic.com/s/roboto/v51/KFOMCnqEu92Fr1ME7kSn66aGLdTylUAMQXC89YmC2DPNWuYjammT.ttf",
    },
    "OpenSans-Regular": {          // Apache 2.0 – clean, neutral sans-serif
      fallback: false,
      data: "https://fonts.gstatic.com/s/opensans/v44/memSYaGs126MiZpBA-UvWbX2vVnXBbObj2OVZyOOSr4dVJWUgsjZ0C4n.ttf",
    },
    "OpenSans-Bold": {             // Apache 2.0
      fallback: false,
      data: "https://fonts.gstatic.com/s/opensans/v44/memSYaGs126MiZpBA-UvWbX2vVnXBbObj2OVZyOOSr4dVJWUgsg-1y4n.ttf",
    },
    "Lato-Regular": {              // OFL – friendly, rounded sans-serif
      fallback: false,
      data: "https://fonts.gstatic.com/s/lato/v25/S6uyw4BMUTPHvxk.ttf",
    },
    "Lato-Bold": {                 // OFL
      fallback: false,
      data: "https://fonts.gstatic.com/s/lato/v25/S6u9w4BMUTPHh6UVew8.ttf",
    },
    // ── Japanese (CJK) ───────────────────────────────────────────────────────
    NotoSerifJP: {                 // OFL
      fallback: false,
      data: "https://fonts.gstatic.com/s/notoserifjp/v30/xn71YHs72GKoTvER4Gn3b5eMRtWGkp6o7MjQ2bwxOubAILO5wBCU.ttf",
    },
    NotoSansJP: {                  // OFL
      fallback: false,
      data: "https://fonts.gstatic.com/s/notosansjp/v53/-F6jfjtqLzI2JPCgQBnw7HFyzSD-AsregP8VFBEj75vY0rw-oME.ttf",
    },
  };
}

// ── Plugins ──────────────────────────────────────────────────────────────────

export const plugins = {
  Text: text,
  MultiVariableText: multiVariableText,
  Image: image,
  UrlImage: urlImage,
  SVG: svg,
  Table: table,
  Line: line,
  Rectangle: rectangle,
  Ellipse: ellipse,
  QRCode: barcodes.qrcode,
};

// ── Placeholders reference ───────────────────────────────────────────────────

export const placeholders: { key: string; label: string }[] = [
  { key: "team_name",     label: "Team-Name" },
  { key: "team_id",       label: "Team-ID" },
  { key: "org",           label: "Schule / Organisation" },
  { key: "org_genitive",  label: "Schule (Genitiv, z.B. \"des Gymnasiums\")" },
  { key: "team_genus",    label: "Genus (der/die/das)" },
  { key: "rank",          label: "Platz (1, 2, 3, ...)" },
  { key: "points_team",   label: "Mannschaftspunkte" },
  { key: "points_player", label: "Brettpunkte" },
  { key: "tie",           label: "Buchholzpunkte" },
  { key: "group",         label: "Wertungsklasse" },
  { key: "event",         label: "Veranstaltungsname" },
];

// ── Data helpers ─────────────────────────────────────────────────────────────

// (Genitiv!)
export const GENUS_MAP: Record<string, string> = { f: "der", m: "des", n: "des" };

/** Build the flat variable map for a result entry. */
export function buildDataMap(
  entry: ResultEntry,
  group_name: string,
  event_name: string,
): Record<string, string> {
  const team = entry.team ?? "";
  return {
    team,
    team_name:     team,
    team_id:       entry.team_id ?? "",
    org:           entry.team_org      ?? "",
    org_genitive:  entry.team_org_genitive ?? entry.team_org ?? "",
    team_genus:    GENUS_MAP[entry.team_genus ?? ""] ?? entry.team_genus ?? "",
    rank:          entry.rank          != null ? String(entry.rank)          : "",
    points_team:   entry.points_team   != null ? String(entry.points_team)   : "",
    points_player: entry.points_player != null ? String(entry.points_player) : "",
    tie:           entry.tie           != null ? String(entry.tie)           : "",
    group:         group_name,
    event:         event_name,
  };
}

/**
 * Build a pdfme input record from a flat data map, driven by the template schema.
 * - multiVariableText fields: value is a JSON string of {variable: value} pairs.
 * - all other fields: value is data[fieldName] when present, otherwise schema content (fallback).
 */
export function buildInput(
  template: Template,
  data: Record<string, string>,
): Record<string, string> {
  const result: Record<string, string> = {};
  for (const page of template.schemas) {
    for (const field of page) {
      if (field.type === "multiVariableText") {
        const vars: Record<string, string> = {};
        for (const v of (field as { variables?: string[] }).variables ?? []) {
          vars[v] = data[v] || " ";
        }
        result[field.name] = JSON.stringify(vars);
      } else if (field.name in data) {
        result[field.name] = data[field.name];
      } else {
        // For fields not in the data map, fall back to schema content and
        // perform {variable} substitution so QR codes / text with placeholders work.
        result[field.name] = (field.content ?? "").replace(
          /\{(\w+)\}/g,
          (_, key) => data[key] ?? `{${key}}`,
        );
      }
    }
  }
  return result;
}
