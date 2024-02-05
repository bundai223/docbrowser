import type { Meta, StoryObj } from '@storybook/react';

import Icon from './index';

const meta: Meta<typeof Icon> = {
  title: 'DocBrowser/Icon',
  component: Icon,
  // ...
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs']
};

type Story = StoryObj<typeof Icon>;
  
export const Primary: Story = {
args: {
  name: 'Hi Sample',
}
};
    
export default meta;