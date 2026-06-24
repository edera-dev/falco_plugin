#!/usr/bin/env python3
"""Merge per-architecture syft CycloneDX scans into one CycloneDX 1.6 SBOM.

syft emits only a single platform per scan, so each architecture of a multi-arch
image is scanned separately. This unions the per-arch component sets into one
SBOM so EVERY architecture's shipped dependencies are covered, then that single
SBOM is attested to the image index digest. (protect's build-and-sign-image
merge reads exactly one CycloneDX predicate from the index digest, so separate
per-arch attestations would not all be seen -- a single unioned SBOM is required.)

Usage:
  merge-cdx.py OUT NAME VERSION PLATFORM=FILE [PLATFORM=FILE ...]

Components are deduped across arches by purl (falling back to bom-ref, then
name@version). Each component records the platform(s) it was found in via a
`dev.edera.platform` property. metadata.component is set to NAME@VERSION so the
downstream merge labels this base cleanly (never name@?).
"""
import copy
import json
import sys


def comp_key(c):
    return (
        c.get("purl")
        or c.get("bom-ref")
        or "%s@%s" % (c.get("name", "?"), c.get("version", ""))
    )


def add_platform_prop(component, platform):
    props = component.setdefault("properties", [])
    for p in props:
        if p.get("name") == "dev.edera.platform":
            p["value"] = ",".join(sorted(set(p["value"].split(",")) | {platform}))
            return
    props.append({"name": "dev.edera.platform", "value": platform})


def main():
    if len(sys.argv) < 5:
        print("usage: merge-cdx.py OUT NAME VERSION PLATFORM=FILE ...", file=sys.stderr)
        sys.exit(2)
    out_path, name, version = sys.argv[1], sys.argv[2], sys.argv[3]
    pairs = sys.argv[4:]

    merged = {}      # comp_key -> kept component dict
    order = []       # first-seen order of comp_keys
    dep_edges = {}   # canonical ref -> set(canonical dependsOn refs)
    roots = set()    # per-arch image root bom-refs (dropped from output graph)

    for pair in pairs:
        platform, _, path = pair.partition("=")
        if not path:
            print("bad PLATFORM=FILE arg: %r" % pair, file=sys.stderr)
            sys.exit(2)
        with open(path) as f:
            doc = json.load(f)

        root = (doc.get("metadata") or {}).get("component", {}).get("bom-ref")
        if root:
            roots.add(root)

        # Map this arch's local bom-refs to the canonical (first-seen) bom-ref,
        # so dependency edges stay valid after dedup.
        local_map = {}
        for c in doc.get("components") or []:
            key = comp_key(c)
            if key not in merged:
                merged[key] = copy.deepcopy(c)
                order.append(key)
            canonical = merged[key].get("bom-ref")
            if c.get("bom-ref") and canonical:
                local_map[c["bom-ref"]] = canonical
            add_platform_prop(merged[key], platform)

        for d in doc.get("dependencies") or []:
            ref = local_map.get(d.get("ref"), d.get("ref"))
            if not ref:
                continue
            deps = {local_map.get(x, x) for x in (d.get("dependsOn") or [])}
            dep_edges.setdefault(ref, set()).update(deps)

    components = [merged[k] for k in order]
    image_ref = "%s@%s" % (name, version) if version else name

    dependencies = [
        {
            "ref": image_ref,
            "dependsOn": [c["bom-ref"] for c in components if c.get("bom-ref")],
        }
    ]
    for ref in sorted(dep_edges):
        if ref in roots or ref == image_ref:
            continue
        deps = sorted(dep_edges[ref])
        if deps:
            dependencies.append({"ref": ref, "dependsOn": deps})

    metadata_component = {"bom-ref": image_ref, "type": "container", "name": name}
    if version:
        metadata_component["version"] = version

    document = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "version": 1,
        "metadata": {"component": metadata_component},
        "components": components,
        "dependencies": dependencies,
    }
    with open(out_path, "w") as out:
        json.dump(document, out, indent=2)
        out.write("\n")

    print(
        json.dumps(
            {
                "component": name,
                "version": version,
                "platforms": [p.split("=", 1)[0] for p in pairs],
                "components": len(components),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
