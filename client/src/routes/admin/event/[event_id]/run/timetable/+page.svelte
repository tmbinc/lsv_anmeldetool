<script>
  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
</script>

For each group, allow to set expected time for ("Registration", "Round #n
paired", "Round #n start", "Ceremony")

<Table>
  <TableHead>
    <TableHeadCell></TableHeadCell>
    <TableHeadCell>WK I</TableHeadCell>
    <TableHeadCell>WK 2</TableHeadCell>
    <TableHeadCell>WK 3</TableHeadCell>
    <TableHeadCell>WK 4</TableHeadCell>
  </TableHead>
  <TableBody>
    <TableBodyRow>
      <TableBodyCell>Anmeldung</TableBodyCell>
      <TableBodyCell>9:00 Uhr</TableBodyCell>
      <TableBodyCell>9:00 Uhr</TableBodyCell>
      <TableBodyCell>9:00 Uhr</TableBodyCell>
      <TableBodyCell>9:00 Uhr</TableBodyCell>
    </TableBodyRow>
    <TableBodyRow>
      <TableBodyCell>Round 1 paired</TableBodyCell>
      <TableBodyCell>9:30 Uhr</TableBodyCell>
      <TableBodyCell>9:30 Uhr</TableBodyCell>
      <TableBodyCell>9:30 Uhr</TableBodyCell>
      <TableBodyCell>9:30 Uhr</TableBodyCell>
    </TableBodyRow>
    <TableBodyRow>
      <TableBodyCell>Round 1 start</TableBodyCell>
      <TableBodyCell>9:35 Uhr</TableBodyCell>
      <TableBodyCell>9:35 Uhr</TableBodyCell>
      <TableBodyCell>9:35 Uhr</TableBodyCell>
      <TableBodyCell>9:40 Uhr</TableBodyCell>
    </TableBodyRow>
    <TableBodyRow>
      <TableBodyCell>Round 2 paired</TableBodyCell>
      <TableBodyCell>10:00 Uhr</TableBodyCell>
      <TableBodyCell>10:00 Uhr</TableBodyCell>
      <TableBodyCell>10:00 Uhr</TableBodyCell>
      <TableBodyCell>10:10 Uhr</TableBodyCell>
    </TableBodyRow>
    <TableBodyRow>
      <TableBodyCell>Ceremony</TableBodyCell>
      <TableBodyCell>16:00 Uhr</TableBodyCell>
      <TableBodyCell>16:00 Uhr</TableBodyCell>
      <TableBodyCell>16:00 Uhr</TableBodyCell>
      <TableBodyCell>16:10 Uhr</TableBodyCell>
    </TableBodyRow>
  </TableBody>
</Table>
<Button>Update</Button>
