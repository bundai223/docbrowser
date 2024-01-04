import type { Meta, StoryObj } from '@storybook/react';

import Button from './Button';

const meta: Meta<typeof Button> = {
  title: 'DocBrowser/Button',
  component: Button,
  // ...
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs']
};

type Story = StoryObj<typeof Button>;
  
export const Primary: Story = {
  args: {
    onClick: e => { console.log('clicked') },
    children: "aaaaaaa"
  },
  render: function Render(args) {
    return <Button>aaaaaaaaa</Button>
  }
};
    
export default meta;