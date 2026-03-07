export interface RcFile {
  /** @see https://envrs.github.io/synopkg/config/synopkgrc/#json */
  $schema?: string;
  /** @see https://envrs.github.io/synopkg/config/custom-types */
  customTypes?: {
    [name: string]: CustomType.Any;
  };
  /** @see https://envrs.github.io/synopkg/config/dependency-groups */
  dependencyGroups?: DependencyGroup[];
  /** @see https://envrs.github.io/synopkg/config/format-bugs */
  formatBugs?: boolean;
  /** @see https://envrs.github.io/synopkg/config/format-repository */
  formatRepository?: boolean;
  /** @see https://envrs.github.io/synopkg/config/indent */
  indent?: string;
  /** @see https://envrs.github.io/synopkg/config/max-concurrent-requests */
  maxConcurrentRequests?: number;
  /** @see https://envrs.github.io/synopkg/semver-groups */
  semverGroups?: SemverGroup.Any[];
  /** @see https://envrs.github.io/synopkg/config/sort-az */
  sortAz?: string[];
  /** @see https://envrs.github.io/synopkg/config/sort-exports */
  sortExports?: string[];
  /** @see https://envrs.github.io/synopkg/config/sort-first */
  sortFirst?: string[];
  /** @see https://envrs.github.io/synopkg/config/sort-packages */
  sortPackages?: boolean;
  /** @see https://envrs.github.io/synopkg/config/source */
  source?: string[];
  /** @see https://envrs.github.io/synopkg/config/strict */
  strict?: boolean;
  /** @see https://envrs.github.io/synopkg/version-groups */
  versionGroups?: VersionGroup.Any[];

  /** @deprecated */
  dependencyTypes?: never;
  /** @deprecated */
  filter?: never;
  /** @deprecated */
  lintFormatting?: never;
  /** @deprecated */
  lintSemverRanges?: never;
  /** @deprecated */
  lintVersions?: never;
  /** @deprecated */
  specifierTypes?: never;
}

export interface GroupSelector {
  /** @see https://envrs.github.io/synopkg/version-groups/highest-semver/#dependencies */
  dependencies?: string[];
  /** @see https://envrs.github.io/synopkg/version-groups/highest-semver/#dependencytypes */
  dependencyTypes?: DependencyType[];
  /** @see https://envrs.github.io/synopkg/version-groups/highest-semver/#label */
  label?: string;
  /** @see https://envrs.github.io/synopkg/version-groups/highest-semver/#packages */
  packages?: string[];
  /** @see https://envrs.github.io/synopkg/version-groups/highest-semver/#specifiertypes */
  specifierTypes?: SpecifierType[];
}

export interface DependencyGroup {
  /** @see https://envrs.github.io/synopkg/config/dependency-groups/#aliasname */
  aliasName: string;
  /** @see https://envrs.github.io/synopkg/config/dependency-groups/#dependencies */
  dependencies?: string[];
  /** @see https://envrs.github.io/synopkg/config/dependency-groups/#dependencytypes */
  dependencyTypes?: DependencyType[];
  /** @see https://envrs.github.io/synopkg/config/dependency-groups/#packages */
  packages?: string[];
  /** @see https://envrs.github.io/synopkg/config/dependency-groups/#specifiertypes */
  specifierTypes?: SpecifierType[];
}

namespace SemverGroup {
  export interface Ignored extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/semver-groups/ignored/#isignored */
    isIgnored: true;
  }
  export interface WithRange extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/semver-groups/with-range/#range */
    range: SemverRange;
  }
  export type Any = Ignored | WithRange;
}

namespace VersionGroup {
  export interface Banned extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/version-groups/banned/#isbanned */
    isBanned: true;
  }
  export interface Ignored extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/version-groups/ignored/#isignored */
    isIgnored: true;
  }
  export interface Pinned extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/version-groups/pinned/#pinversion */
    pinVersion: string;
  }
  export interface SnappedTo extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/version-groups/snapped-to/#snapto */
    snapTo: string[];
  }
  export interface SameRange extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/version-groups/same-range/#policy */
    policy: 'sameRange';
  }
  export interface SameMinor extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/version-groups/same-minor/#policy */
    policy: 'sameMinor';
  }
  export interface Standard extends GroupSelector {
    /** @see https://envrs.github.io/synopkg/version-groups/lowest-semver/#preferversion */
    preferVersion?: 'highestSemver' | 'lowestSemver';
  }
  export type Any = Banned | Ignored | Pinned | SameRange | SameMinor | SnappedTo | Standard;
}

namespace CustomType {
  export interface NameAndVersionProps {
    /** @see https://envrs.github.io/synopkg/config/custom-types/#namepath */
    namePath: string;
    /** @see https://envrs.github.io/synopkg/config/custom-types/#name */
    path: string;
    /** @see https://envrs.github.io/synopkg/config/custom-types/#namestrategy */
    strategy: 'name~version';
  }
  export interface NamedVersionString {
    /** @see https://envrs.github.io/synopkg/config/custom-types/#name */
    path: string;
    /** @see https://envrs.github.io/synopkg/config/custom-types/#namestrategy */
    strategy: 'name@version';
  }
  export interface UnnamedVersionString {
    /** @see https://envrs.github.io/synopkg/config/custom-types/#name */
    path: string;
    /** @see https://envrs.github.io/synopkg/config/custom-types/#namestrategy */
    strategy: 'version';
  }
  export interface VersionsByName {
    /** @see https://envrs.github.io/synopkg/config/custom-types/#name */
    path: string;
    /** @see https://envrs.github.io/synopkg/config/custom-types/#namestrategy */
    strategy: 'versionsByName';
  }
  export type Any = NameAndVersionProps | NamedVersionString | UnnamedVersionString | VersionsByName;
}

type SemverRange = '' | '*' | '>' | '>=' | '.x' | '<' | '<=' | '^' | '~';

type DependencyType = 'dev' | 'local' | 'overrides' | 'peer' | 'pnpmOverrides' | 'prod' | 'resolutions' | AnyString;

type SpecifierType =
  | 'alias'
  | 'exact'
  | 'file'
  | 'git'
  | 'latest'
  | 'major'
  | 'minor'
  | 'missing'
  | 'range'
  | 'range-complex'
  | 'range-major'
  | 'range-minor'
  | 'tag'
  | 'unsupported'
  | 'url'
  | 'workspace-protocol'
  | AnyString;

type AnyString = string & {};

/** Each Instance printed by `synopkg json` */
export type JsonOutput = {
  dependency: string;
  dependencyGroup: string;
  dependencyType: DependencyType;
  package: string;
  property: ['dependencies'];
  strategy: CustomType.Any['strategy'];
  versionGroup: VersionGroupVariant;
  preferredSemverRange: SemverRange | null;
  statusCode: StatusCode;
  actual: {
    raw: string;
    type: SpecifierType;
  };
  expected: {
    raw: string;
    type: SpecifierType;
  } | null;
};

export type VersionGroupVariant =
  | 'Banned'
  | 'HighestSemver'
  | 'Ignored'
  | 'LowestSemver'
  | 'Pinned'
  | 'SameRange'
  | 'SameMinor'
  | 'SnappedTo';

export type StatusCode =
  | 'IsHighestOrLowestSemver'
  | 'IsIdenticalToLocal'
  | 'IsIdenticalToPin'
  | 'IsIdenticalToSnapTarget'
  | 'IsIgnored'
  | 'IsLocalAndValid'
  | 'IsNonSemverButIdentical'
  | 'SatisfiesHighestOrLowestSemver'
  | 'SatisfiesLocal'
  | 'SatisfiesSameRangeGroup'
  | 'SatisfiesSameMinorGroup'
  | 'SatisfiesSnapTarget'
  | 'DiffersToHighestOrLowestSemver'
  | 'DiffersToLocal'
  | 'DiffersToNpmRegistry'
  | 'DiffersToPin'
  | 'DiffersToSnapTarget'
  | 'IsBanned'
  | 'PinOverridesSemverRange'
  | 'PinOverridesSemverRangeMismatch'
  | 'SameMinorOverridesSemverRange'
  | 'SameMinorOverridesSemverRangeMismatch'
  | 'SemverRangeMismatch'
  | 'DependsOnInvalidLocalPackage'
  | 'NonSemverMismatch'
  | 'SameRangeMismatch'
  | 'SameMinorMismatch'
  | 'DependsOnMissingSnapTarget'
  | 'InvalidLocalVersion'
  | 'RefuseToBanLocal'
  | 'RefuseToPinLocal'
  | 'RefuseToSnapLocal'
  | 'MatchConflictsWithHighestOrLowestSemver'
  | 'MatchConflictsWithLocal'
  | 'MatchConflictsWithSnapTarget'
  | 'MismatchConflictsWithHighestOrLowestSemver'
  | 'MismatchConflictsWithLocal'
  | 'MismatchConflictsWithSnapTarget';
